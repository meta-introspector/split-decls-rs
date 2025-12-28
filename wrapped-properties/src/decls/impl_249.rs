macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesShort!();
        PropertyNamesShortBorrowed!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > PropertyNamesShort < T > { # [doc = " Creates a new instance of `PropertyNamesShortBorrowed<T>`."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub fn new () -> PropertyNamesShortBorrowed < 'static , T > { PropertyNamesShortBorrowed :: new () } # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable (provider : & (impl DataProvider < T :: DataMarkerShort > + ? Sized) ,) -> Result < Self , DataError > { Ok (Self { map : provider . load (Default :: default ()) ? . payload . cast () , }) } # [doc = " Construct a borrowed version of this type that can be queried."] # [doc = ""] # [doc = " This avoids a potential small underlying cost per API call (like `get_static()`) by consolidating it"] # [doc = " up front."] # [inline] pub fn as_borrowed (& self) -> PropertyNamesShortBorrowed < '_ , T > { PropertyNamesShortBorrowed { map : T :: nep_short_identity (self . map . get ()) , } } }
    };
}

impl_249!()