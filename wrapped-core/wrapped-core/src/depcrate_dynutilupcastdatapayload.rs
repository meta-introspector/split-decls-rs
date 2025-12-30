// Generated macro for UpcastDataPayload (trait)
macro_rules! Depcrate_dynutilUpcastDataPayload {
() => {
// Module: crate::dynutil
// Provides: {"UpcastDataPayload"}
// Dependencies: {}
# [doc = " Trait to allow conversion from `DataPayload<T>` to `DataPayload<S>`."] # [doc = ""] # [doc = " This trait can be manually implemented in order to enable [`impl_dynamic_data_provider`]."] pub trait UpcastDataPayload < M > where M : crate :: DynamicDataMarker , Self : Sized + crate :: DynamicDataMarker , { # [doc = " Upcast a `DataPayload<T>` to a `DataPayload<S>` where `T` implements trait `S`."] fn upcast (other : crate :: DataPayload < M >) -> crate :: DataPayload < Self > ; }
};
}
