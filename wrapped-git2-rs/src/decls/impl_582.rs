macro_rules! deps {
    () => {
        PushUpdate!();
        Oid!();
        Binding!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl PushUpdate < '_ > { # [doc = " Returns the source name of the reference as a byte slice."] pub fn src_refname_bytes (& self) -> & [u8] { unsafe { crate :: opt_bytes (self , (* self . raw) . src_refname) . unwrap () } } # [doc = " Returns the source name of the reference, or None if it is not valid UTF-8."] pub fn src_refname (& self) -> Option < & str > { str :: from_utf8 (self . src_refname_bytes ()) . ok () } # [doc = " Returns the name of the reference to update on the server as a byte slice."] pub fn dst_refname_bytes (& self) -> & [u8] { unsafe { crate :: opt_bytes (self , (* self . raw) . dst_refname) . unwrap () } } # [doc = " Returns the name of the reference to update on the server, or None if it is not valid UTF-8."] pub fn dst_refname (& self) -> Option < & str > { str :: from_utf8 (self . dst_refname_bytes ()) . ok () } # [doc = " Returns the current target of the reference."] pub fn src (& self) -> Oid { unsafe { Binding :: from_raw (& (* self . raw) . src as * const _) } } # [doc = " Returns the new target for the reference."] pub fn dst (& self) -> Oid { unsafe { Binding :: from_raw (& (* self . raw) . dst as * const _) } } }
    };
}

impl_582!();