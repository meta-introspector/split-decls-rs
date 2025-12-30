// Generated macro for impl_71 (impl)
macro_rules! Depcrate_server_nameimpl_71 {
() => {
// Module: crate::server_name
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'a > DnsName < 'a > { # [doc = " Produce a borrowed `DnsName` from this owned `DnsName`."] pub fn borrow (& 'a self) -> Self { Self (match self { Self (DnsNameInner :: Borrowed (s)) => DnsNameInner :: Borrowed (s) , # [cfg (feature = "alloc")] Self (DnsNameInner :: Owned (s)) => DnsNameInner :: Borrowed (s . as_str ()) , }) } # [doc = " Copy this object to produce an owned `DnsName`, smashing the case to lowercase"] # [doc = " in one operation."] # [cfg (feature = "alloc")] pub fn to_lowercase_owned (& self) -> DnsName < 'static > { DnsName (DnsNameInner :: Owned (self . as_ref () . to_ascii_lowercase ())) } # [doc = " Produce an owned `DnsName` from this (potentially borrowed) `DnsName`."] # [cfg (feature = "alloc")] pub fn to_owned (& self) -> DnsName < 'static > { DnsName (DnsNameInner :: Owned (match self { Self (DnsNameInner :: Borrowed (s)) => s . to_string () , # [cfg (feature = "alloc")] Self (DnsNameInner :: Owned (s)) => s . clone () , })) } # [cfg (feature = "alloc")] fn try_from_string (s : String) -> Result < Self , String > { match validate (s . as_bytes ()) { Ok (_) => Ok (Self (DnsNameInner :: Owned (s))) , Err (_) => Err (s) , } } # [doc = " Produces a borrowed [`DnsName`] from a borrowed [`str`]."] pub const fn try_from_str (s : & str) -> Result < DnsName < '_ > , InvalidDnsNameError > { match validate (s . as_bytes ()) { Ok (_) => Ok (DnsName (DnsNameInner :: Borrowed (s))) , Err (err) => Err (err) , } } }
};
}
