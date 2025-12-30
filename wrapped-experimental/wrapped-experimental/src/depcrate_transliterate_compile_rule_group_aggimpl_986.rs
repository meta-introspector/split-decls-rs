// Generated macro for impl_986 (impl)
macro_rules! Depcrate_transliterate_compile_rule_group_aggimpl_986 {
() => {
// Module: crate::transliterate::compile::rule_group_agg
// Provides: {"impl_986"}
// Dependencies: {}
impl < 'p > ReverseRuleGroup < 'p > { fn new_conversion (rule : UniConversionRule < 'p >) -> Self { Self :: Conversion (vec ! [rule]) } fn new_transform (rule : Cow < 'p , parse :: SingleId >) -> Self { let mut group = VecDeque :: new () ; group . push_front (rule) ; Self :: Transform (group) } fn push (& mut self , rule : UniRule < 'p >) -> Option < Self > { match (& mut * self , rule) { (Self :: Conversion (group) , UniRule :: Conversion (rule)) => { group . push (rule) ; None } (Self :: Transform (group) , UniRule :: Transform (rule)) => { group . push_front (rule) ; None } (Self :: Conversion (_) , UniRule :: Transform (new_rule)) => { Some (core :: mem :: replace (self , Self :: new_transform (new_rule))) } (Self :: Transform (_) , UniRule :: Conversion (new_rule)) => { Some (core :: mem :: replace (self , Self :: new_conversion (new_rule))) } } } }
};
}
