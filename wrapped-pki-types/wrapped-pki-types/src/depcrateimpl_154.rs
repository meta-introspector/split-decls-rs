// Generated macro for impl_154 (impl)
macro_rules! Depcrateimpl_154 {
() => {
// Module: crate
// Provides: {"impl_154"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl TryFrom < Vec < u8 > > for PrivateKeyDer < '_ > { type Error = & 'static str ; fn try_from (key : Vec < u8 >) -> Result < Self , Self :: Error > { Ok (match PrivateKeyDer :: try_from (& key [..]) ? { PrivateKeyDer :: Pkcs1 (_) => Self :: Pkcs1 (key . into ()) , PrivateKeyDer :: Sec1 (_) => Self :: Sec1 (key . into ()) , PrivateKeyDer :: Pkcs8 (_) => Self :: Pkcs8 (key . into ()) , }) } }
};
}
