macro_rules! deps {
    () => {
        Result!();
        ExtensionContext!();
        ValidationResult!();
        NextValidation!();
        ServerError!();
    };
}

macro_rules! impl_587 {
    () => {
        deps!();
        impl NextValidation < '_ > { # [doc = " Call the [Extension::validation] function of next extension."] pub async fn run (self , ctx : & ExtensionContext < '_ > ,) -> Result < ValidationResult , Vec < ServerError > > { if let Some ((first , next)) = self . chain . split_first () { first . validation (ctx , NextValidation { chain : next , validation_fut : self . validation_fut , } ,) . await } else { self . validation_fut . await } } }
    };
}

impl_587!();