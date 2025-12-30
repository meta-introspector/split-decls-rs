// Generated macro for impl_19 (impl)
macro_rules! Depcrate_baseimpl_19 {
() => {
// Module: crate::base
// Provides: {"impl_19"}
// Dependencies: {}
impl < K , V > Index < usize > for Tower < K , V > { type Output = Atomic < Node < K , V > > ; fn index (& self , index : usize) -> & Atomic < Node < K , V > > { unsafe { & * (& self . pointers as * const Atomic < Node < K , V > >) . add (index) } } }
};
}
