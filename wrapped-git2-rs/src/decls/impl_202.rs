macro_rules! deps {
    () => {
        Oid!();
        BlameHunk!();
        Signature!();
        Note!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < 'blame > BlameHunk < 'blame > { unsafe fn from_raw_const (raw : * const raw :: git_blame_hunk) -> BlameHunk < 'blame > { BlameHunk { raw : raw as * mut raw :: git_blame_hunk , _marker : marker :: PhantomData , } } # [doc = " Returns OID of the commit where this line was last changed"] pub fn final_commit_id (& self) -> Oid { unsafe { Oid :: from_raw (& (* self . raw) . final_commit_id) } } # [doc = " Returns signature of the commit."] pub fn final_signature (& self) -> Signature < '_ > { unsafe { signature :: from_raw_const (self , (* self . raw) . final_signature) } } # [doc = " Returns line number where this hunk begins."] # [doc = ""] # [doc = " Note that the start line is counting from 1."] pub fn final_start_line (& self) -> usize { unsafe { (* self . raw) . final_start_line_number } } # [doc = " Returns the OID of the commit where this hunk was found."] # [doc = ""] # [doc = " This will usually be the same as `final_commit_id`,"] # [doc = " except when `BlameOptions::track_copies_any_commit_copies` has been"] # [doc = " turned on"] pub fn orig_commit_id (& self) -> Oid { unsafe { Oid :: from_raw (& (* self . raw) . orig_commit_id) } } # [doc = " Returns signature of the commit."] pub fn orig_signature (& self) -> Signature < '_ > { unsafe { signature :: from_raw_const (self , (* self . raw) . orig_signature) } } # [doc = " Returns line number where this hunk begins."] # [doc = ""] # [doc = " Note that the start line is counting from 1."] pub fn orig_start_line (& self) -> usize { unsafe { (* self . raw) . orig_start_line_number } } # [doc = " Returns path to the file where this hunk originated."] # [doc = ""] # [doc = " Note: `None` could be returned for non-unicode paths on Windows."] pub fn path (& self) -> Option < & Path > { unsafe { if let Some (bytes) = crate :: opt_bytes (self , (* self . raw) . orig_path) { Some (util :: bytes2path (bytes)) } else { None } } } # [doc = " Tests whether this hunk has been tracked to a boundary commit"] # [doc = " (the root, or the commit specified in git_blame_options.oldest_commit)."] pub fn is_boundary (& self) -> bool { unsafe { (* self . raw) . boundary == 1 } } # [doc = " Returns number of lines in this hunk."] pub fn lines_in_hunk (& self) -> usize { unsafe { (* self . raw) . lines_in_hunk as usize } } }
    };
}

impl_202!();