// Generated macro for parse_fragment_for_element (function)
macro_rules! Depcrate_driverparse_fragment_for_element {
() => {
// Module: crate::driver
// Provides: {"parse_fragment_for_element"}
// Dependencies: {}
# [doc = " Like `parse_fragment`, but with an existing context element"] # [doc = " and optionally a form element."] pub fn parse_fragment_for_element < Sink > (sink : Sink , opts : ParseOpts , context_element : Sink :: Handle , form_element : Option < Sink :: Handle >) -> Parser < Sink > where Sink : TreeSink { let tb = TreeBuilder :: new_for_fragment (sink , context_element , form_element , opts . tree_builder) ; let tok_opts = TokenizerOpts { initial_state : Some (tb . tokenizer_state_for_context_elem ()) , .. opts . tokenizer } ; let tok = Tokenizer :: new (tb , tok_opts) ; Parser { tokenizer : tok } }
};
}
