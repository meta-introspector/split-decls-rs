// Generated macro for impl_1281 (impl)
macro_rules! Depcrate_stackimpl_1281 {
() => {
// Module: crate::stack
// Provides: {"impl_1281"}
// Dependencies: {}
impl < T : Stackable > IndexMut < usize > for StackRef < T > { fn index_mut (& mut self , index : usize) -> & mut T :: Ref { self . get_mut (index) . unwrap () } }
};
}
