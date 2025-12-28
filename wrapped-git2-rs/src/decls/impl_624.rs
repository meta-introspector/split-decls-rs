macro_rules! deps {
    () => {
        Signature!();
        Oid!();
        ReflogEntry!();
        Binding!();
    };
}

macro_rules! impl_624 {
    () => {
        deps!();
        impl < 'reflog > ReflogEntry < 'reflog > { # [doc = " Get the committer of this entry"] pub fn committer (& self) -> Signature < '_ > { unsafe { let ptr = raw :: git_reflog_entry_committer (self . raw) ; signature :: from_raw_const (self , ptr) } } # [doc = " Get the new oid"] pub fn id_new (& self) -> Oid { unsafe { Binding :: from_raw (raw :: git_reflog_entry_id_new (self . raw)) } } # [doc = " Get the old oid"] pub fn id_old (& self) -> Oid { unsafe { Binding :: from_raw (raw :: git_reflog_entry_id_old (self . raw)) } } # [doc = " Get the log message, returning `None` on invalid UTF-8."] pub fn message (& self) -> Option < & str > { self . message_bytes () . and_then (| s | str :: from_utf8 (s) . ok ()) } # [doc = " Get the log message as a byte array."] pub fn message_bytes (& self) -> Option < & [u8] > { unsafe { crate :: opt_bytes (self , raw :: git_reflog_entry_message (self . raw)) } } }
    };
}

impl_624!()