// Generated macro for impl_31 (impl)
macro_rules! Depcrate_code_point_mapimpl_31 {
() => {
// Module: crate::code_point_map
// Provides: {"impl_31"}
// Dependencies: {}
impl < T : TrieValue > CodePointMapDataBorrowed < 'static , T > { # [doc = " Creates a new [`CodePointMapDataBorrowed`] for a [`EnumeratedProperty`]."] # [doc = ""] # [doc = " See the documentation on [`EnumeratedProperty`] implementations for details."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub const fn new () -> Self where T : EnumeratedProperty , { CodePointMapDataBorrowed { map : T :: SINGLETON } } # [doc = " Cheaply converts a [`CodePointMapDataBorrowed<'static>`] into a [`CodePointMapData`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`CodePointMapData`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`CodePointMapDataBorrowed`]."] pub const fn static_to_owned (self) -> CodePointMapData < T > { CodePointMapData { data : DataPayload :: from_static_ref (self . map) , } } }
};
}
