// Generated macro for redirect (macro)
macro_rules! Depcrate_transliterate_compileredirect {
() => {
// Module: crate::transliterate::compile
// Provides: {"redirect"}
// Dependencies: {}
macro_rules ! redirect { ($ ($ marker : ty) ,*) => { $ (impl < PP : ? Sized , NP : ? Sized + DataProvider <$ marker >, NC : ? Sized > DataProvider <$ marker > for RuleCollectionProvider <'_ , PP , NP , NC > { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { self . normalizer_provider . load (req) } }) * } }
};
}
