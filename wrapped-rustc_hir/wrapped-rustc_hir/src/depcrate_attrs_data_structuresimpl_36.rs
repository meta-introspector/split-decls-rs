// Generated macro for impl_36 (impl)
macro_rules! Depcrate_attrs_data_structuresimpl_36 {
() => {
// Module: crate::attrs::data_structures
// Provides: {"impl_36"}
// Dependencies: {}
impl Deprecation { # [doc = " Whether an item marked with #[deprecated(since = \"X\")] is currently"] # [doc = " deprecated (i.e., whether X is not greater than the current rustc"] # [doc = " version)."] pub fn is_in_effect (& self) -> bool { match self . since { DeprecatedSince :: RustcVersion (since) => since <= RustcVersion :: CURRENT , DeprecatedSince :: Future => false , DeprecatedSince :: NonStandard (_) => true , DeprecatedSince :: Unspecified | DeprecatedSince :: Err => true , } } pub fn is_since_rustc_version (& self) -> bool { matches ! (self . since , DeprecatedSince :: RustcVersion (_)) } }
};
}
