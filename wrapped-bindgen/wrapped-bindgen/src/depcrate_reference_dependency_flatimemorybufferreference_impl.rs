// Generated macro for IMemoryBufferReference_Impl (trait)
macro_rules! Depcrate_reference_dependency_flatIMemoryBufferReference_Impl {
() => {
// Module: crate::reference_dependency_flat
// Provides: {"IMemoryBufferReference_Impl"}
// Dependencies: {}
pub trait IMemoryBufferReference_Impl : IClosable_Impl { fn Capacity (& self) -> windows_core :: Result < u32 > ; fn RemoveClosed (& self , cookie : i64) -> windows_core :: Result < () > ; }
};
}
