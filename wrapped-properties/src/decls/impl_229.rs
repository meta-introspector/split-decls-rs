macro_rules! deps {
    () => {
        PropertyParser!();
        ParseableEnumeratedProperty!();
        PropertyParserBorrowed!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < T > PropertyParser < T > { # [doc = " Creates a new instance of `PropertyParser<T>` using compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub fn new () -> PropertyParserBorrowed < 'static , T > where T : ParseableEnumeratedProperty , { PropertyParserBorrowed :: new () } # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable (provider : & (impl DataProvider < T :: DataMarker > + ? Sized) ,) -> Result < Self , DataError > where T : ParseableEnumeratedProperty , { Ok (Self { map : provider . load (Default :: default ()) ? . payload . cast () , markers : PhantomData , }) } # [doc = " Construct a borrowed version of this type that can be queried."] # [doc = ""] # [doc = " This avoids a potential small underlying cost per API call (like `get_strict()`) by consolidating it"] # [doc = " up front."] # [inline] pub fn as_borrowed (& self) -> PropertyParserBorrowed < '_ , T > { PropertyParserBorrowed { map : self . map . get () , markers : PhantomData , } } # [doc (hidden)] pub fn erase (self) -> PropertyParser < u16 > { PropertyParser { map : self . map . cast () , markers : PhantomData , } } }
    };
}

impl_229!()