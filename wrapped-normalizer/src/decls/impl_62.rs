macro_rules! deps {
    () => {
        Uts46Mapper!();
        ComposingNormalizer!();
        Uts46MapperBorrowed!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Uts46Mapper { # [doc = " Constructs a borrowed version of this type for more efficient querying."] pub fn as_borrowed (& self) -> Uts46MapperBorrowed < '_ > { Uts46MapperBorrowed { normalizer : self . normalizer . as_borrowed () , } } # [doc = " Construct with compiled data."] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub const fn new () -> Uts46MapperBorrowed < 'static > { Uts46MapperBorrowed :: new () } # [doc = " Construct with provider."] # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new < D > (provider : & D) -> Result < Self , DataError > where D : DataProvider < NormalizerUts46DataV1 > + DataProvider < NormalizerNfdTablesV1 > + DataProvider < NormalizerNfkdTablesV1 > + DataProvider < NormalizerNfcV1 > + ? Sized , { let normalizer = ComposingNormalizer :: try_new_uts46_unstable (provider) ? ; Ok (Uts46Mapper { normalizer }) } }
    };
}

impl_62!()