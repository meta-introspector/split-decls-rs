// Generated macro for expand_ensures_tts (function)
macro_rules! Depcrate_contractsexpand_ensures_tts {
() => {
// Module: crate::contracts
// Provides: {"expand_ensures_tts"}
// Dependencies: {}
fn expand_ensures_tts (ecx : & mut ExtCtxt < '_ > , attr_span : Span , annotation : TokenStream , annotated : TokenStream ,) -> Result < TokenStream , ErrorGuaranteed > { let feature_span = ecx . with_def_site_ctxt (attr_span) ; expand_contract_clause (ecx , attr_span , annotated , | new_tts | { new_tts . push_tree (TokenTree :: Token (token :: Token :: from_ast_ident (Ident :: new (kw :: ContractEnsures , feature_span)) , Spacing :: Joint ,)) ; new_tts . push_tree (TokenTree :: Delimited (DelimSpan :: from_single (attr_span) , DelimSpacing :: new (Spacing :: JointHidden , Spacing :: JointHidden) , token :: Delimiter :: Parenthesis , annotation ,)) ; Ok (()) }) }
};
}
