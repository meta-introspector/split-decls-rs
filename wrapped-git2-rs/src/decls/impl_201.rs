macro_rules! deps {
    () => {
        BlameIter!();
        BlameHunk!();
        Binding!();
        Blame!();
        Error!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < 'repo > Blame < 'repo > { # [doc = " Get blame data for a file that has been modified in memory."] # [doc = ""] # [doc = " Lines that differ between the buffer and the committed version are"] # [doc = " marked as having a zero OID for their final_commit_id."] pub fn blame_buffer (& self , buffer : & [u8]) -> Result < Blame < '_ > , Error > { let mut raw = ptr :: null_mut () ; unsafe { try_call ! (raw :: git_blame_buffer (& mut raw , self . raw , buffer . as_ptr () as * const c_char , buffer . len ())) ; Ok (Binding :: from_raw (raw)) } } # [doc = " Gets the number of hunks that exist in the blame structure."] pub fn len (& self) -> usize { unsafe { raw :: git_blame_get_hunk_count (self . raw) as usize } } # [doc = " Return `true` is there is no hunk in the blame structure."] pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Gets the blame hunk at the given index."] pub fn get_index (& self , index : usize) -> Option < BlameHunk < '_ > > { unsafe { let ptr = raw :: git_blame_get_hunk_byindex (self . raw () , index as u32) ; if ptr . is_null () { None } else { Some (BlameHunk :: from_raw_const (ptr)) } } } # [doc = " Gets the hunk that relates to the given line number in the newest"] # [doc = " commit."] pub fn get_line (& self , lineno : usize) -> Option < BlameHunk < '_ > > { unsafe { let ptr = raw :: git_blame_get_hunk_byline (self . raw () , lineno) ; if ptr . is_null () { None } else { Some (BlameHunk :: from_raw_const (ptr)) } } } # [doc = " Returns an iterator over the hunks in this blame."] pub fn iter (& self) -> BlameIter < '_ > { BlameIter { range : 0 .. self . len () , blame : self , } } }
    };
}

impl_201!();