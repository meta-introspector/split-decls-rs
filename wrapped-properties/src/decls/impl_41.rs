macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesLongBorrowed!();
        PropertyNamesLong!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > PropertyNamesLong < T > { # [doc = " Creates a new instance of `PropertyNamesLongBorrowed<T>`."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub fn new () -> PropertyNamesLongBorrowed < 'static , T > { PropertyNamesLongBorrowed :: new () } # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable (provider : & (impl DataProvider < T :: DataMarkerLong > + ? Sized) ,) -> Result < Self , DataError > { Ok (Self { map : provider . load (Default :: default ()) ? . payload . cast () , }) } # [doc = " Construct a borrowed version of this type that can be queried."] # [doc = ""] # [doc = " This avoids a potential small underlying cost per API call (like `get_static()`) by consolidating it"] # [doc = " up front."] # [inline] pub fn as_borrowed (& self) -> PropertyNamesLongBorrowed < '_ , T > { PropertyNamesLongBorrowed { map : T :: nep_long_identity (self . map . get ()) , } } }
    };
}

impl_41!();