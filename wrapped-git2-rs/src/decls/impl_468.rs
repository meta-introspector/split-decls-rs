macro_rules! deps {
    () => {
        Note!();
        Oid!();
        Binding!();
        Signature!();
    };
}

macro_rules! impl_468 {
    () => {
        deps!();
        impl < 'repo > Note < 'repo > { # [doc = " Get the note author"] pub fn author (& self) -> Signature < '_ > { unsafe { signature :: from_raw_const (self , raw :: git_note_author (& * self . raw)) } } # [doc = " Get the note committer"] pub fn committer (& self) -> Signature < '_ > { unsafe { signature :: from_raw_const (self , raw :: git_note_committer (& * self . raw)) } } # [doc = " Get the note message, in bytes."] pub fn message_bytes (& self) -> & [u8] { unsafe { crate :: opt_bytes (self , raw :: git_note_message (& * self . raw)) . unwrap () } } # [doc = " Get the note message as a string, returning `None` if it is not UTF-8."] pub fn message (& self) -> Option < & str > { str :: from_utf8 (self . message_bytes ()) . ok () } # [doc = " Get the note object's id"] pub fn id (& self) -> Oid { unsafe { Binding :: from_raw (raw :: git_note_id (& * self . raw)) } } }
    };
}

impl_468!()