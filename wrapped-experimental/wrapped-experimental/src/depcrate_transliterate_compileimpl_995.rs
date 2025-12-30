// Generated macro for impl_995 (impl)
macro_rules! Depcrate_transliterate_compileimpl_995 {
() => {
// Module: crate::transliterate::compile
// Provides: {"impl_995"}
// Dependencies: {}
impl < PP : ? Sized , NP : ? Sized , NC : ? Sized + DataProvider < CaseMapV1 > > DataProvider < CaseMapV1 > for RuleCollectionProvider < '_ , PP , NP , NC > { fn load (& self , req : DataRequest) -> Result < DataResponse < CaseMapV1 > , DataError > { self . casemap_provider . load (req) } }
};
}
