// Generated macro for impl_840 (impl)
macro_rules! Depcrate_extensionsimpl_840 {
() => {
// Module: crate::extensions
// Provides: {"impl_840"}
// Dependencies: {}
impl NextValidation < '_ > { # [doc = " Call the [Extension::validation] function of next extension."] pub async fn run (self , ctx : & ExtensionContext < '_ > ,) -> Result < ValidationResult , Vec < ServerError > > { if let Some ((first , next)) = self . chain . split_first () { first . validation (ctx , NextValidation { chain : next , validation_fut : self . validation_fut , } ,) . await } else { self . validation_fut . await } } }
};
}
