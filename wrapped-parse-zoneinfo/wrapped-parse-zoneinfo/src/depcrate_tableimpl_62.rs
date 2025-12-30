// Generated macro for impl_62 (impl)
macro_rules! Depcrate_tableimpl_62 {
() => {
// Module: crate::table
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'line > From < line :: Rule < 'line > > for RuleInfo { fn from (info : line :: Rule) -> RuleInfo { RuleInfo { from_year : info . from_year , to_year : info . to_year , month : info . month , day : info . day , time : info . time . 0 . as_seconds () , time_type : info . time . 1 , time_to_add : info . time_to_add . as_seconds () , letters : info . letters . map (str :: to_owned) , } } }
};
}
