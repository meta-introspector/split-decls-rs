macro_rules! deps {
    () => {
        Statuses!();
        StatusIter!();
        Binding!();
        StatusEntry!();
    };
}

macro_rules! impl_751 {
    () => {
        deps!();
        impl < 'repo > Statuses < 'repo > { # [doc = " Gets a status entry from this list at the specified index."] # [doc = ""] # [doc = " Returns `None` if the index is out of bounds."] pub fn get (& self , index : usize) -> Option < StatusEntry < '_ > > { unsafe { let p = raw :: git_status_byindex (self . raw , index as size_t) ; Binding :: from_raw_opt (p) } } # [doc = " Gets the count of status entries in this list."] # [doc = ""] # [doc = " If there are no changes in status (according to the options given"] # [doc = " when the status list was created), this should return 0."] pub fn len (& self) -> usize { unsafe { raw :: git_status_list_entrycount (self . raw) as usize } } # [doc = " Return `true` if there is no status entry in this list."] pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Returns an iterator over the statuses in this list."] pub fn iter (& self) -> StatusIter < '_ > { StatusIter { statuses : self , range : 0 .. self . len () , } } }
    };
}

impl_751!()