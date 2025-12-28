macro_rules! deps {
    () => {
        TranslationBundleError!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl From < (FluentResource , Vec < ParserError >) > for TranslationBundleError { fn from ((_ , mut errs) : (FluentResource , Vec < ParserError >)) -> Self { TranslationBundleError :: ParseFtl (errs . pop () . expect ("failed ftl parse with no errors")) } }
    };
}

impl_34!();