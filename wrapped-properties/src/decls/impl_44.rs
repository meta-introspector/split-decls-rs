macro_rules! deps {
    () => {
        PropertyNamesLongBorrowed!();
        PropertyNamesLong!();
        NamedEnumeratedProperty!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > PropertyNamesLongBorrowed < 'static , T > { # [doc = " Creates a new instance of `PropertyNamesLongBorrowed<T>`."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub fn new () -> Self { Self { map : T :: SINGLETON_LONG , } } # [doc = " Cheaply converts a [`PropertyNamesLongBorrowed<'static>`] into a [`PropertyNamesLong`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`PropertyNamesLong`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`PropertyNamesLongBorrowed`]."] # [doc = ""] # [doc = " This is currently not `const` unlike other `static_to_owned()` functions since it needs"] # [doc = " const traits to do that safely"] pub fn static_to_owned (self) -> PropertyNamesLong < T > { PropertyNamesLong { map : DataPayload :: from_static_ref (T :: nep_long_identity_static (self . map)) , } } }
    };
}

impl_44!()