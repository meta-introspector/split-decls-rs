// Generated macro for impl_11 (impl)
macro_rules! Depcrate_code_point_setimpl_11 {
() => {
// Module: crate::code_point_set
// Provides: {"impl_11"}
// Dependencies: {}
impl CodePointSetDataBorrowed < 'static > { # [doc = " Creates a new [`CodePointSetData`] for a [`BinaryProperty`]."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [inline] # [cfg (feature = "compiled_data")] pub const fn new < P : BinaryProperty > () -> Self { CodePointSetDataBorrowed { set : P :: SINGLETON } } # [doc = " Cheaply converts a [`CodePointSetDataBorrowed<'static>`] into a [`CodePointSetData`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`CodePointSetData`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`CodePointSetDataBorrowed`]."] pub const fn static_to_owned (self) -> CodePointSetData { CodePointSetData { data : DataPayload :: from_static_ref (self . set) , } } }
};
}
