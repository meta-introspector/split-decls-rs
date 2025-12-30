// Generated macro for impl_936 (impl)
macro_rules! Depcrate_transliterate_compile_pass1impl_936 {
() => {
// Module: crate::transliterate::compile::pass1
// Provides: {"impl_936"}
// Dependencies: {}
impl SpecialConstructCounts { pub (crate) fn num_total (& self) -> usize { self . num_compounds + self . num_quantifiers_opt + self . num_quantifiers_kleene + self . num_quantifiers_kleene_plus + self . num_segments + self . num_unicode_sets + self . num_function_calls + self . max_left_placeholders as usize + self . max_right_placeholders as usize + self . max_backref_num as usize } fn combine (& mut self , other : Self) { let Self { num_compounds , num_quantifiers_opt , num_quantifiers_kleene , num_quantifiers_kleene_plus , num_segments , num_unicode_sets , num_function_calls , max_left_placeholders , max_right_placeholders , max_backref_num , } = other ; self . num_compounds += num_compounds ; self . num_quantifiers_opt += num_quantifiers_opt ; self . num_quantifiers_kleene += num_quantifiers_kleene ; self . num_quantifiers_kleene_plus += num_quantifiers_kleene_plus ; self . num_segments += num_segments ; self . num_unicode_sets += num_unicode_sets ; self . num_function_calls += num_function_calls ; self . max_left_placeholders = self . max_left_placeholders . max (max_left_placeholders) ; self . max_right_placeholders = self . max_right_placeholders . max (max_right_placeholders) ; self . max_backref_num = self . max_backref_num . max (max_backref_num) ; } }
};
}
