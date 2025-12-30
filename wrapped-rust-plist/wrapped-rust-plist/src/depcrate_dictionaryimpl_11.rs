// Generated macro for impl_11 (impl)
macro_rules! Depcrate_dictionaryimpl_11 {
() => {
// Module: crate::dictionary
// Provides: {"impl_11"}
// Dependencies: {}
impl Extend < (String , Value) > for Dictionary { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = (String , Value) > , { self . map . extend (iter) ; } }
};
}
