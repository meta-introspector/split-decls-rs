macro_rules! deps {
    () => {
        PropertyParserBorrowed!();
        ParseableEnumeratedProperty!();
        PropertyParser!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl < T : TrieValue > PropertyParserBorrowed < 'static , T > { # [doc = " Creates a new instance of `PropertyParserBorrowed<T>` using compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub fn new () -> Self where T : ParseableEnumeratedProperty , { Self { map : T :: SINGLETON , markers : PhantomData , } } # [doc = " Cheaply converts a [`PropertyParserBorrowed<'static>`] into a [`PropertyParser`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`PropertyParser`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`PropertyParserBorrowed`]."] pub const fn static_to_owned (self) -> PropertyParser < T > { PropertyParser { map : DataPayload :: from_static_ref (self . map) , markers : PhantomData , } } }
    };
}

impl_232!();