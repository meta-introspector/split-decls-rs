macro_rules! deps {
    () => {
        Oid!();
        Reflog!();
        Commit!();
        Signature!();
        Note!();
        Error!();
        Transaction!();
    };
}

macro_rules! impl_808 {
    () => {
        deps!();
        impl < 'repo > Transaction < 'repo > { # [doc = " Lock the specified reference by name."] pub fn lock_ref (& mut self , refname : & str) -> Result < () , Error > { let refname = CString :: new (refname) . unwrap () ; unsafe { try_call ! (raw :: git_transaction_lock_ref (self . raw , refname)) ; } Ok (()) } # [doc = " Set the target of the specified reference."] # [doc = ""] # [doc = " The reference must have been locked via `lock_ref`."] # [doc = ""] # [doc = " If `reflog_signature` is `None`, the [`Signature`] is read from the"] # [doc = " repository config."] pub fn set_target (& mut self , refname : & str , target : Oid , reflog_signature : Option < & Signature < '_ > > , reflog_message : & str ,) -> Result < () , Error > { let refname = CString :: new (refname) . unwrap () ; let reflog_message = CString :: new (reflog_message) . unwrap () ; unsafe { try_call ! (raw :: git_transaction_set_target (self . raw , refname , target . raw () , reflog_signature . map (| s | s . raw ()) , reflog_message)) ; } Ok (()) } # [doc = " Set the target of the specified symbolic reference."] # [doc = ""] # [doc = " The reference must have been locked via `lock_ref`."] # [doc = ""] # [doc = " If `reflog_signature` is `None`, the [`Signature`] is read from the"] # [doc = " repository config."] pub fn set_symbolic_target (& mut self , refname : & str , target : & str , reflog_signature : Option < & Signature < '_ > > , reflog_message : & str ,) -> Result < () , Error > { let refname = CString :: new (refname) . unwrap () ; let target = CString :: new (target) . unwrap () ; let reflog_message = CString :: new (reflog_message) . unwrap () ; unsafe { try_call ! (raw :: git_transaction_set_symbolic_target (self . raw , refname , target , reflog_signature . map (| s | s . raw ()) , reflog_message)) ; } Ok (()) } # [doc = " Add a [`Reflog`] to the transaction."] # [doc = ""] # [doc = " This commit the in-memory [`Reflog`] to disk when the transaction commits."] # [doc = " Note that atomicity is **not* guaranteed: if the transaction fails to"] # [doc = " modify `refname`, the reflog may still have been committed to disk."] # [doc = ""] # [doc = " If this is combined with setting the target, that update won't be"] # [doc = " written to the log (i.e. the `reflog_signature` and `reflog_message`"] # [doc = " parameters will be ignored)."] pub fn set_reflog (& mut self , refname : & str , reflog : Reflog) -> Result < () , Error > { let refname = CString :: new (refname) . unwrap () ; unsafe { try_call ! (raw :: git_transaction_set_reflog (self . raw , refname , reflog . raw ())) ; } Ok (()) } # [doc = " Remove a reference."] # [doc = ""] # [doc = " The reference must have been locked via `lock_ref`."] pub fn remove (& mut self , refname : & str) -> Result < () , Error > { let refname = CString :: new (refname) . unwrap () ; unsafe { try_call ! (raw :: git_transaction_remove (self . raw , refname)) ; } Ok (()) } # [doc = " Commit the changes from the transaction."] # [doc = ""] # [doc = " The updates will be made one by one, and the first failure will stop the"] # [doc = " processing."] pub fn commit (self) -> Result < () , Error > { unsafe { try_call ! (raw :: git_transaction_commit (self . raw)) ; } Ok (()) } }
    };
}

impl_808!()