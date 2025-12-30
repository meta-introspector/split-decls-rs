// Generated macro for impl_463 (impl)
macro_rules! Depcrate_lazy_bufferimpl_463 {
() => {
// Module: crate::lazy_buffer
// Provides: {"impl_463"}
// Dependencies: {}
impl < I , J > Index < J > for LazyBuffer < I > where I : Iterator , I :: Item : Sized , Vec < I :: Item > : Index < J > , { type Output = < Vec < I :: Item > as Index < J > > :: Output ; fn index (& self , index : J) -> & Self :: Output { self . buffer . index (index) } }
};
}
