// Generated macro for TokenSink (trait)
macro_rules! Depcrate_tokenizer_interfaceTokenSink {
() => {
// Module: crate::tokenizer::interface
// Provides: {"TokenSink"}
// Dependencies: {}
# [doc = " Types which can receive tokens from the tokenizer."] pub trait TokenSink { # [doc = " Process a token."] fn process_token (& mut self , token : Token) ; # [doc = " Used in the markup declaration open state. By default, this always"] # [doc = " returns false and thus all CDATA sections are tokenized as bogus"] # [doc = " comments."] # [doc = " https://html.spec.whatwg.org/multipage/#markup-declaration-open-state"] fn adjusted_current_node_present_but_not_in_html_namespace (& self) -> bool { false } # [doc = " The tokenizer will call this after emitting any tag."] # [doc = " This allows the tree builder to change the tokenizer's state."] # [doc = " By default no state changes occur."] fn query_state_change (& mut self) -> Option < states :: State > { None } }
};
}
