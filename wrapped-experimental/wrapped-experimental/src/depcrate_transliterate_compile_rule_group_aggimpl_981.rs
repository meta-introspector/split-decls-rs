// Generated macro for impl_981 (impl)
macro_rules! Depcrate_transliterate_compile_rule_group_aggimpl_981 {
() => {
// Module: crate::transliterate::compile::rule_group_agg
// Provides: {"impl_981"}
// Dependencies: {}
impl < 'p > ForwardRuleGroup < 'p > { fn new_conversion (rule : UniConversionRule < 'p >) -> Self { Self :: Conversion (vec ! [rule]) } fn new_transform (rule : Cow < 'p , parse :: SingleId >) -> Self { Self :: Transform (vec ! [rule]) } fn push (& mut self , rule : UniRule < 'p >) -> Option < Self > { match (& mut * self , rule) { (Self :: Conversion (group) , UniRule :: Conversion (rule)) => { group . push (rule) ; None } (Self :: Transform (group) , UniRule :: Transform (rule)) => { group . push (rule) ; None } (Self :: Conversion (_) , UniRule :: Transform (new_rule)) => { Some (core :: mem :: replace (self , Self :: new_transform (new_rule))) } (Self :: Transform (_) , UniRule :: Conversion (new_rule)) => { Some (core :: mem :: replace (self , Self :: new_conversion (new_rule))) } } } }
};
}
