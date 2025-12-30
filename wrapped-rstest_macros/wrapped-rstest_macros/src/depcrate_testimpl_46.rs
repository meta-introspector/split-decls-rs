// Generated macro for impl_46 (impl)
macro_rules! Depcrate_testimpl_46 {
() => {
// Module: crate::test
// Provides: {"impl_46"}
// Dependencies: {}
impl RsTestInfo { pub fn push_case (& mut self , case : TestCase) { self . data . items . push (RsTestItem :: TestCase (case)) ; } pub fn extend (& mut self , cases : impl Iterator < Item = TestCase >) { self . data . items . extend (cases . map (RsTestItem :: TestCase)) ; } }
};
}
