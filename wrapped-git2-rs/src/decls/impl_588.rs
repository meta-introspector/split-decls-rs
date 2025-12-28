macro_rules! deps {
    () => {
        Error!();
        Oid!();
        Index!();
        RebaseOperation!();
        Binding!();
        Rebase!();
        Signature!();
    };
}

macro_rules! impl_588 {
    () => {
        deps!();
        impl < 'repo > Rebase < 'repo > { # [doc = " Gets the count of rebase operations that are to be applied."] pub fn len (& self) -> usize { unsafe { raw :: git_rebase_operation_entrycount (self . raw) } } # [doc = " Gets the original `HEAD` ref name for merge rebases."] pub fn orig_head_name (& self) -> Option < & str > { let name_bytes = unsafe { crate :: opt_bytes (self , raw :: git_rebase_orig_head_name (self . raw)) } ; name_bytes . and_then (| s | str :: from_utf8 (s) . ok ()) } # [doc = " Gets the original HEAD id for merge rebases."] pub fn orig_head_id (& self) -> Option < Oid > { unsafe { Oid :: from_raw_opt (raw :: git_rebase_orig_head_id (self . raw)) } } # [doc = "  Gets the rebase operation specified by the given index."] pub fn nth (& mut self , n : usize) -> Option < RebaseOperation < '_ > > { unsafe { let op = raw :: git_rebase_operation_byindex (self . raw , n) ; if op . is_null () { None } else { Some (RebaseOperation :: from_raw (op)) } } } # [doc = " Gets the index of the rebase operation that is currently being applied."] # [doc = " If the first operation has not yet been applied (because you have called"] # [doc = " `init` but not yet `next`) then this returns None."] pub fn operation_current (& mut self) -> Option < usize > { let cur = unsafe { raw :: git_rebase_operation_current (self . raw) } ; if cur == raw :: GIT_REBASE_NO_OPERATION { None } else { Some (cur) } } # [doc = " Gets the index produced by the last operation, which is the result of"] # [doc = " `next()` and which will be committed by the next invocation of"] # [doc = " `commit()`. This is useful for resolving conflicts in an in-memory"] # [doc = " rebase before committing them."] # [doc = ""] # [doc = " This is only applicable for in-memory rebases; for rebases within a"] # [doc = " working directory, the changes were applied to the repository's index."] pub fn inmemory_index (& mut self) -> Result < Index , Error > { let mut idx = ptr :: null_mut () ; unsafe { try_call ! (raw :: git_rebase_inmemory_index (& mut idx , self . raw)) ; Ok (Binding :: from_raw (idx)) } } # [doc = " Commits the current patch.  You must have resolved any conflicts that"] # [doc = " were introduced during the patch application from the `git_rebase_next`"] # [doc = " invocation. To keep the author and message from the original commit leave"] # [doc = " them as None"] pub fn commit (& mut self , author : Option < & Signature < '_ > > , committer : & Signature < '_ > , message : Option < & str > ,) -> Result < Oid , Error > { let mut id : raw :: git_oid = unsafe { mem :: zeroed () } ; let message = crate :: opt_cstr (message) ? ; unsafe { try_call ! (raw :: git_rebase_commit (& mut id , self . raw , author . map (| a | a . raw ()) , committer . raw () , ptr :: null () , message)) ; Ok (Binding :: from_raw (& id as * const _)) } } # [doc = " Aborts a rebase that is currently in progress, resetting the repository"] # [doc = " and working directory to their state before rebase began."] pub fn abort (& mut self) -> Result < () , Error > { unsafe { try_call ! (raw :: git_rebase_abort (self . raw)) ; } Ok (()) } # [doc = " Finishes a rebase that is currently in progress once all patches have"] # [doc = " been applied."] pub fn finish (& mut self , signature : Option < & Signature < '_ > >) -> Result < () , Error > { unsafe { try_call ! (raw :: git_rebase_finish (self . raw , signature . map (| s | s . raw ()))) ; } Ok (()) } }
    };
}

impl_588!()