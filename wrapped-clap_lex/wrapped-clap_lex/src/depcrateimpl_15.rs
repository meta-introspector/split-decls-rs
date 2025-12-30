// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < I , T > From < I > for RawArgs where I : Iterator < Item = T > , T : Into < OsString > , { fn from (val : I) -> Self { Self { items : val . map (| x | x . into ()) . collect () , } } }
};
}
