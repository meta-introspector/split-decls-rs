// Generated macro for impl_243 (impl)
macro_rules! Depcrate_parse_rstestimpl_243 {
() => {
// Module: crate::parse::rstest
// Provides: {"impl_243"}
// Dependencies: {}
impl RsTestData { pub (crate) fn case_args (& self) -> impl Iterator < Item = & Pat > { self . items . iter () . filter_map (| it | match it { RsTestItem :: CaseArgName (ref arg) => Some (arg) , _ => None , }) } # [allow (dead_code)] pub (crate) fn has_case_args (& self) -> bool { self . case_args () . next () . is_some () } pub (crate) fn cases (& self) -> impl Iterator < Item = & TestCase > { self . items . iter () . filter_map (| it | match it { RsTestItem :: TestCase (ref case) => Some (case) , _ => None , }) } pub (crate) fn has_cases (& self) -> bool { self . cases () . next () . is_some () } pub (crate) fn fixtures (& self) -> impl Iterator < Item = & Fixture > { self . items . iter () . filter_map (| it | match it { RsTestItem :: Fixture (ref fixture) => Some (fixture) , _ => None , }) } # [allow (dead_code)] pub (crate) fn has_fixtures (& self) -> bool { self . fixtures () . next () . is_some () } pub (crate) fn list_values (& self) -> impl Iterator < Item = & ValueList > { self . items . iter () . filter_map (| mv | match mv { RsTestItem :: ValueList (ref value_list) => Some (value_list) , _ => None , }) } pub (crate) fn has_list_values (& self) -> bool { self . list_values () . next () . is_some () } }
};
}
