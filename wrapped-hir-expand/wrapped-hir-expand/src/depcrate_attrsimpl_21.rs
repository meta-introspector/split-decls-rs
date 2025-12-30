// Generated macro for impl_21 (impl)
macro_rules! Depcrate_attrsimpl_21 {
() => {
// Module: crate::attrs
// Provides: {"impl_21"}
// Dependencies: {}
impl AttrId { const INNER_ATTR_SET_BIT : u32 = 1 << 31 ; pub fn new (id : usize , is_inner : bool) -> Self { assert ! (id <= ! Self :: INNER_ATTR_SET_BIT as usize) ; let id = id as u32 ; Self { id : if is_inner { id | Self :: INNER_ATTR_SET_BIT } else { id } } } pub fn ast_index (& self) -> usize { (self . id & ! Self :: INNER_ATTR_SET_BIT) as usize } pub fn is_inner_attr (& self) -> bool { self . id & Self :: INNER_ATTR_SET_BIT != 0 } }
};
}
