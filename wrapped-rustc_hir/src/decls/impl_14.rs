macro_rules! deps {
    () => {
        DeprecatedSince!();
        Deprecation!();
        RustcVersion!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Deprecation { # [doc = " Whether an item marked with #[deprecated(since = \"X\")] is currently"] # [doc = " deprecated (i.e., whether X is not greater than the current rustc"] # [doc = " version)."] pub fn is_in_effect (& self) -> bool { match self . since { DeprecatedSince :: RustcVersion (since) => since <= RustcVersion :: CURRENT , DeprecatedSince :: Future => false , DeprecatedSince :: NonStandard (_) => true , DeprecatedSince :: Unspecified | DeprecatedSince :: Err => true , } } pub fn is_since_rustc_version (& self) -> bool { matches ! (self . since , DeprecatedSince :: RustcVersion (_)) } }
    };
}

impl_14!()