macro_rules! deps {
    () => {
        CodePointMapData!();
        EnumeratedProperty!();
        CodePointMapDataBorrowed!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : TrieValue > CodePointMapDataBorrowed < 'static , T > { # [doc = " Creates a new [`CodePointMapDataBorrowed`] for a [`EnumeratedProperty`]."] # [doc = ""] # [doc = " See the documentation on [`EnumeratedProperty`] implementations for details."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub const fn new () -> Self where T : EnumeratedProperty , { CodePointMapDataBorrowed { map : T :: SINGLETON } } # [doc = " Cheaply converts a [`CodePointMapDataBorrowed<'static>`] into a [`CodePointMapData`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`CodePointMapData`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`CodePointMapDataBorrowed`]."] pub const fn static_to_owned (self) -> CodePointMapData < T > { CodePointMapData { data : DataPayload :: from_static_ref (self . map) , } } }
    };
}

impl_15!()