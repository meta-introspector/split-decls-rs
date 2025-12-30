// Generated macro for impl_1280 (impl)
macro_rules! Depcrate_stackimpl_1280 {
() => {
// Module: crate::stack
// Provides: {"impl_1280"}
// Dependencies: {}
impl < T : Stackable > Index < usize > for StackRef < T > { type Output = T :: Ref ; fn index (& self , index : usize) -> & T :: Ref { self . get (index) . unwrap () } }
};
}
