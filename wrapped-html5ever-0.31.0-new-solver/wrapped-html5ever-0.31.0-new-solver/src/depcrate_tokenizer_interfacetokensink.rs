// Generated macro for TokenSink (trait)
macro_rules! Depcrate_tokenizer_interfaceTokenSink {
() => {
// Module: crate::tokenizer::interface
// Provides: {"TokenSink"}
// Dependencies: {}
# [doc = " Types which can receive tokens from the tokenizer."] pub trait TokenSink { type Handle ; # [doc = " Process a token."] fn process_token (& self , token : Token , line_number : u64) -> TokenSinkResult < Self :: Handle > ; fn end (& self) { } # [doc = " Used in the markup declaration open state. By default, this always"] # [doc = " returns false and thus all CDATA sections are tokenized as bogus"] # [doc = " comments."] # [doc = " <https://html.spec.whatwg.org/multipage/#markup-declaration-open-state>"] fn adjusted_current_node_present_but_not_in_html_namespace (& self) -> bool { false } }
};
}
