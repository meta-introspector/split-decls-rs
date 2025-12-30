// Generated macro for impl_160 (impl)
macro_rules! Depcrateimpl_160 {
() => {
// Module: crate
// Provides: {"impl_160"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl From < Vec < u8 > > for PrivatePkcs1KeyDer < '_ > { fn from (vec : Vec < u8 >) -> Self { Self (Der (BytesInner :: Owned (vec))) } }
};
}
