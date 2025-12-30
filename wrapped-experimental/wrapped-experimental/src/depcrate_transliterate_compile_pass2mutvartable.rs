// Generated macro for MutVarTable (struct)
macro_rules! Depcrate_transliterate_compile_pass2MutVarTable {
() => {
// Module: crate::transliterate::compile::pass2
// Provides: {"MutVarTable"}
// Dependencies: {}
struct MutVarTable { compounds : MutVarTableField < String > , quantifiers_opt : MutVarTableField < String > , quantifiers_kleene : MutVarTableField < String > , quantifiers_kleene_plus : MutVarTableField < String > , segments : MutVarTableField < ds :: Segment < 'static > > , unicode_sets : MutVarTableField < parse :: UnicodeSet > , function_calls : MutVarTableField < ds :: FunctionCall < 'static > > , left_placeholder_base : u32 , right_placeholder_base : u32 , backref_base : u32 , counts : pass1 :: SpecialConstructCounts , }
};
}
