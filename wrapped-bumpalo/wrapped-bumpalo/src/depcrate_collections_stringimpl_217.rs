// Generated macro for impl_217 (impl)
macro_rules! Depcrate_collections_stringimpl_217 {
() => {
// Module: crate::collections::string
// Provides: {"impl_217"}
// Dependencies: {}
impl < 'bump > ops :: Index < ops :: Range < usize > > for String < 'bump > { type Output = str ; # [inline] fn index (& self , index : ops :: Range < usize >) -> & str { & self [..] [index] } }
};
}
