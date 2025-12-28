macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesShort!();
        PropertyNamesShortBorrowed!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > PropertyNamesShortBorrowed < 'static , T > { # [doc = " Creates a new instance of `PropertyNamesShortBorrowed<T>`."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub fn new () -> Self { Self { map : T :: SINGLETON_SHORT , } } # [doc = " Cheaply converts a [`PropertyNamesShortBorrowed<'static>`] into a [`PropertyNamesShort`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`PropertyNamesShort`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`PropertyNamesShortBorrowed`]."] # [doc = ""] # [doc = " This is currently not `const` unlike other `static_to_owned()` functions since it needs"] # [doc = " const traits to do that safely"] pub fn static_to_owned (self) -> PropertyNamesShort < T > { PropertyNamesShort { map : DataPayload :: from_static_ref (T :: nep_short_identity_static (self . map)) , } } }
    };
}

impl_54!();