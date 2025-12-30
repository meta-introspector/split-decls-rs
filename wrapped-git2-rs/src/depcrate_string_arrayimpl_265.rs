// Generated macro for impl_265 (impl)
macro_rules! Depcrate_string_arrayimpl_265 {
() => {
// Module: crate::string_array
// Provides: {"impl_265"}
// Dependencies: {}
impl Binding for StringArray { type Raw = raw :: git_strarray ; unsafe fn from_raw (raw : raw :: git_strarray) -> StringArray { StringArray { raw } } fn raw (& self) -> raw :: git_strarray { self . raw } }
};
}
