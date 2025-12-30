// Generated macro for impl_174 (impl)
macro_rules! Depcrateimpl_174 {
() => {
// Module: crate
// Provides: {"impl_174"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl From < Vec < u8 > > for PrivatePkcs8KeyDer < '_ > { fn from (vec : Vec < u8 >) -> Self { Self (Der (BytesInner :: Owned (vec))) } }
};
}
