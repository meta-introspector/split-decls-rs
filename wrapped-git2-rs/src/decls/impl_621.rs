macro_rules! deps {
    () => {
        ReflogEntry!();
        Oid!();
        Error!();
        ReflogIter!();
        Signature!();
        Reflog!();
        Binding!();
    };
}

macro_rules! impl_621 {
    () => {
        deps!();
        impl Reflog { # [doc = " Add a new entry to the in-memory reflog."] pub fn append (& mut self , new_oid : Oid , committer : & Signature < '_ > , msg : Option < & str > ,) -> Result < () , Error > { let msg = crate :: opt_cstr (msg) ? ; unsafe { try_call ! (raw :: git_reflog_append (self . raw , new_oid . raw () , committer . raw () , msg)) ; } Ok (()) } # [doc = " Remove an entry from the reflog by its index"] # [doc = ""] # [doc = " To ensure there's no gap in the log history, set rewrite_previous_entry"] # [doc = " param value to `true`. When deleting entry n, member old_oid of entry"] # [doc = " n-1 (if any) will be updated with the value of member new_oid of entry"] # [doc = " n+1."] pub fn remove (& mut self , i : usize , rewrite_previous_entry : bool) -> Result < () , Error > { unsafe { try_call ! (raw :: git_reflog_drop (self . raw , i as size_t , rewrite_previous_entry)) ; } Ok (()) } # [doc = " Lookup an entry by its index"] # [doc = ""] # [doc = " Requesting the reflog entry with an index of 0 (zero) will return the"] # [doc = " most recently created entry."] pub fn get (& self , i : usize) -> Option < ReflogEntry < '_ > > { unsafe { let ptr = raw :: git_reflog_entry_byindex (self . raw , i as size_t) ; Binding :: from_raw_opt (ptr) } } # [doc = " Get the number of log entries in a reflog"] pub fn len (& self) -> usize { unsafe { raw :: git_reflog_entrycount (self . raw) as usize } } # [doc = " Return `true ` is there is no log entry in a reflog"] pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Get an iterator to all entries inside of this reflog"] pub fn iter (& self) -> ReflogIter < '_ > { ReflogIter { range : 0 .. self . len () , reflog : self , } } # [doc = " Write an existing in-memory reflog object back to disk using an atomic"] # [doc = " file lock."] pub fn write (& mut self) -> Result < () , Error > { unsafe { try_call ! (raw :: git_reflog_write (self . raw)) ; } Ok (()) } }
    };
}

impl_621!();