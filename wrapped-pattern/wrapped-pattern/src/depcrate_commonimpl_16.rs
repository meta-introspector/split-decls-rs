// Generated macro for impl_16 (impl)
macro_rules! Depcrate_commonimpl_16 {
() => {
// Module: crate::common
// Provides: {"impl_16"}
// Dependencies: {}
impl < K , T > PlaceholderValueProvider < K > for & '_ T where T : PlaceholderValueProvider < K > + ? Sized , { type Error = T :: Error ; type W < 'a > = T :: W < 'a > where Self : 'a ; type L < 'a , 'l > = T :: L < 'a , 'l > where Self : 'a ; fn value_for (& self , key : K) -> Self :: W < '_ > { (* self) . value_for (key) } fn map_literal < 'a , 'l > (& 'a self , literal : & 'l str) -> Self :: L < 'a , 'l > { (* self) . map_literal (literal) } }
};
}
