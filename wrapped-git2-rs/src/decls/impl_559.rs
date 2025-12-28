macro_rules! deps {
    () => {
        PathspecDiffEntries!();
        PathspecFailedEntries!();
        DiffDelta!();
        Binding!();
        PathspecEntries!();
        PathspecMatchList!();
    };
}

macro_rules! impl_559 {
    () => {
        deps!();
        impl < 'ps > PathspecMatchList < 'ps > { fn entrycount (& self) -> usize { unsafe { raw :: git_pathspec_match_list_entrycount (& * self . raw) as usize } } fn failed_entrycount (& self) -> usize { unsafe { raw :: git_pathspec_match_list_failed_entrycount (& * self . raw) as usize } } # [doc = " Returns an iterator over the matching filenames in this list."] pub fn entries (& self) -> PathspecEntries < '_ > { let n = self . entrycount () ; let n = if n > 0 && self . entry (0) . is_none () { 0 } else { n } ; PathspecEntries { range : 0 .. n , list : self , } } # [doc = " Get a matching filename by position."] # [doc = ""] # [doc = " If this list was generated from a diff, then the return value will"] # [doc = " always be `None."] pub fn entry (& self , i : usize) -> Option < & [u8] > { unsafe { let ptr = raw :: git_pathspec_match_list_entry (& * self . raw , i as size_t) ; crate :: opt_bytes (self , ptr) } } # [doc = " Returns an iterator over the matching diff entries in this list."] pub fn diff_entries (& self) -> PathspecDiffEntries < '_ > { let n = self . entrycount () ; let n = if n > 0 && self . diff_entry (0) . is_none () { 0 } else { n } ; PathspecDiffEntries { range : 0 .. n , list : self , } } # [doc = " Get a matching diff delta by position."] # [doc = ""] # [doc = " If the list was not generated from a diff, then the return value will"] # [doc = " always be `None`."] pub fn diff_entry (& self , i : usize) -> Option < DiffDelta < '_ > > { unsafe { let ptr = raw :: git_pathspec_match_list_diff_entry (& * self . raw , i as size_t) ; Binding :: from_raw_opt (ptr as * mut _) } } # [doc = " Returns an iterator over the non-matching entries in this list."] pub fn failed_entries (& self) -> PathspecFailedEntries < '_ > { let n = self . failed_entrycount () ; let n = if n > 0 && self . failed_entry (0) . is_none () { 0 } else { n } ; PathspecFailedEntries { range : 0 .. n , list : self , } } # [doc = " Get an original pathspec string that had no matches."] pub fn failed_entry (& self , i : usize) -> Option < & [u8] > { unsafe { let ptr = raw :: git_pathspec_match_list_failed_entry (& * self . raw , i as size_t) ; crate :: opt_bytes (self , ptr) } } }
    };
}

impl_559!()