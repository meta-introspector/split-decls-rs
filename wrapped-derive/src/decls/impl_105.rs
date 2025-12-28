macro_rules! deps {
    () => {
        GeneratorError!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl GeneratorError { pub fn write_errors (self) -> TokenStream { match self { GeneratorError :: Syn (err) => err . to_compile_error () , GeneratorError :: Darling (err) => err . write_errors () , } } }
    };
}

impl_105!();