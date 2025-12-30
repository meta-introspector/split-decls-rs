// Generated macro for impl_18 (impl)
macro_rules! Depcrate_visitimpl_18 {
() => {
// Module: crate::visit
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a > ReplaceGenericType < 'a > { pub fn new (generic_type : & 'a str , arg_type : & 'a PathSegment) -> Self { Self { generic_type , arg_type , } } pub fn replace_generic_type (item : & mut Item , generic_type : & 'a str , arg_type : & 'a PathSegment) { let mut s = Self :: new (generic_type , arg_type) ; s . visit_item_mut (item) ; } }
};
}
