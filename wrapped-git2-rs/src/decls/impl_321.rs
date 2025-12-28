macro_rules! deps {
    () => {
        DiffLine!();
        DiffFindOptions!();
        Commit!();
        DiffFormatEmailOptions!();
        DiffHunk!();
        BinaryCb!();
        Buf!();
        Oid!();
        DiffDelta!();
        DiffFormat!();
        LineCb!();
        HunkCb!();
        Diff!();
        DiffPatchidOptions!();
        PrintCb!();
        Binding!();
        DiffCallbacks!();
        Deltas!();
        FileCb!();
        Error!();
        DiffStats!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl < 'repo > Diff < 'repo > { # [doc = " Merge one diff into another."] # [doc = ""] # [doc = " This merges items from the \"from\" list into the \"self\" list.  The"] # [doc = " resulting diff will have all items that appear in either list."] # [doc = " If an item appears in both lists, then it will be \"merged\" to appear"] # [doc = " as if the old version was from the \"onto\" list and the new version"] # [doc = " is from the \"from\" list (with the exception that if the item has a"] # [doc = " pending DELETE in the middle, then it will show as deleted)."] pub fn merge (& mut self , from : & Diff < 'repo >) -> Result < () , Error > { unsafe { try_call ! (raw :: git_diff_merge (self . raw , &* from . raw)) ; } Ok (()) } # [doc = " Returns an iterator over the deltas in this diff."] pub fn deltas (& self) -> Deltas < '_ > { let num_deltas = unsafe { raw :: git_diff_num_deltas (& * self . raw) } ; Deltas { range : 0 .. (num_deltas as usize) , diff : self , } } # [doc = " Return the diff delta for an entry in the diff list."] pub fn get_delta (& self , i : usize) -> Option < DiffDelta < '_ > > { unsafe { let ptr = raw :: git_diff_get_delta (& * self . raw , i as size_t) ; Binding :: from_raw_opt (ptr as * mut _) } } # [doc = " Check if deltas are sorted case sensitively or insensitively."] pub fn is_sorted_icase (& self) -> bool { unsafe { raw :: git_diff_is_sorted_icase (& * self . raw) == 1 } } # [doc = " Iterate over a diff generating formatted text output."] # [doc = ""] # [doc = " Returning `false` from the callback will terminate the iteration and"] # [doc = " return an error from this function."] pub fn print < F > (& self , format : DiffFormat , mut cb : F) -> Result < () , Error > where F : FnMut (DiffDelta < '_ > , Option < DiffHunk < '_ > > , DiffLine < '_ >) -> bool , { let mut cb : & mut PrintCb < '_ > = & mut cb ; let ptr = & mut cb as * mut _ ; let print : raw :: git_diff_line_cb = Some (print_cb) ; unsafe { try_call ! (raw :: git_diff_print (self . raw , format , print , ptr as * mut _)) ; Ok (()) } } # [doc = " Loop over all deltas in a diff issuing callbacks."] # [doc = ""] # [doc = " Returning `false` from any callback will terminate the iteration and"] # [doc = " return an error from this function."] pub fn foreach (& self , file_cb : & mut FileCb < '_ > , binary_cb : Option < & mut BinaryCb < '_ > > , hunk_cb : Option < & mut HunkCb < '_ > > , line_cb : Option < & mut LineCb < '_ > > ,) -> Result < () , Error > { let mut cbs = DiffCallbacks { file : Some (file_cb) , binary : binary_cb , hunk : hunk_cb , line : line_cb , } ; let ptr = & mut cbs as * mut _ ; unsafe { let binary_cb_c : raw :: git_diff_binary_cb = if cbs . binary . is_some () { Some (binary_cb_c) } else { None } ; let hunk_cb_c : raw :: git_diff_hunk_cb = if cbs . hunk . is_some () { Some (hunk_cb_c) } else { None } ; let line_cb_c : raw :: git_diff_line_cb = if cbs . line . is_some () { Some (line_cb_c) } else { None } ; let file_cb : raw :: git_diff_file_cb = Some (file_cb_c) ; try_call ! (raw :: git_diff_foreach (self . raw , file_cb , binary_cb_c , hunk_cb_c , line_cb_c , ptr as * mut _)) ; Ok (()) } } # [doc = " Accumulate diff statistics for all patches."] pub fn stats (& self) -> Result < DiffStats , Error > { let mut ret = ptr :: null_mut () ; unsafe { try_call ! (raw :: git_diff_get_stats (& mut ret , self . raw)) ; Ok (Binding :: from_raw (ret)) } } # [doc = " Transform a diff marking file renames, copies, etc."] # [doc = ""] # [doc = " This modifies a diff in place, replacing old entries that look like"] # [doc = " renames or copies with new entries reflecting those changes. This also"] # [doc = " will, if requested, break modified files into add/remove pairs if the"] # [doc = " amount of change is above a threshold."] pub fn find_similar (& mut self , opts : Option < & mut DiffFindOptions >) -> Result < () , Error > { let opts = opts . map (| opts | & opts . raw) ; unsafe { try_call ! (raw :: git_diff_find_similar (self . raw , opts)) ; } Ok (()) } # [doc = " Create an e-mail ready patch from a diff."] # [doc = ""] # [doc = " Matches the format created by `git format-patch`"] # [doc (hidden)] # [deprecated (note = "refactored to `Email::from_diff` to match upstream")] pub fn format_email (& mut self , patch_no : usize , total_patches : usize , commit : & crate :: Commit < 'repo > , opts : Option < & mut DiffFormatEmailOptions > ,) -> Result < Buf , Error > { assert ! (patch_no > 0) ; assert ! (patch_no <= total_patches) ; let mut default = DiffFormatEmailOptions :: default () ; let raw_opts = opts . map_or (& mut default . raw , | opts | & mut opts . raw) ; let summary = commit . summary_bytes () . unwrap () ; let mut message = commit . message_bytes () ; assert ! (message . starts_with (summary)) ; message = & message [summary . len () ..] ; raw_opts . patch_no = patch_no ; raw_opts . total_patches = total_patches ; let id = commit . id () ; raw_opts . id = id . raw () ; raw_opts . summary = summary . as_ptr () as * const _ ; raw_opts . body = message . as_ptr () as * const _ ; raw_opts . author = commit . author () . raw () ; let buf = Buf :: new () ; # [allow (deprecated)] unsafe { try_call ! (raw :: git_diff_format_email (buf . raw () , self . raw , &* raw_opts)) ; } Ok (buf) } # [doc = " Create a patch ID from a diff."] pub fn patchid (& self , opts : Option < & mut DiffPatchidOptions >) -> Result < Oid , Error > { let mut raw = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; unsafe { try_call ! (raw :: git_diff_patchid (& mut raw , self . raw , opts . map (| o | & mut o . raw))) ; Ok (Binding :: from_raw (& raw as * const _)) } } }
    };
}

impl_321!();