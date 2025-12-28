macro_rules! deps {
    () => {
        TranslationBundleError!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl From < Vec < FluentError > > for TranslationBundleError { fn from (mut errs : Vec < FluentError >) -> Self { TranslationBundleError :: AddResource (errs . pop () . expect ("failed adding resource to bundle with no errors") ,) } }
    };
}

impl_7!()