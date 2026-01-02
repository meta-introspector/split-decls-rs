mkuse!{use rustc_ast :: token ;}
mkuse!{use rustc_ast :: tokenstream :: { DelimSpacing , DelimSpan , Spacing , TokenStream , TokenTree } ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_expand :: base :: { AttrProcMacro , ExtCtxt } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_span :: symbol :: { Ident , Symbol , kw } ;}
mkitem!{mkstruct!{pub (crate) struct ExpandRequires ;}}
mkitem!{mkstruct!{pub (crate) struct ExpandEnsures ;}}
mkitem!{mkimpl!{impl AttrProcMacro for ExpandRequires { fn expand < 'cx > (& self , ecx : & 'cx mut ExtCtxt < '_ > , span : Span , annotation : TokenStream , annotated : TokenStream ,) -> Result < TokenStream , ErrorGuaranteed > { expand_requires_tts (ecx , span , annotation , annotated) } }}}
mkitem!{mkimpl!{impl AttrProcMacro for ExpandEnsures { fn expand < 'cx > (& self , ecx : & 'cx mut ExtCtxt < '_ > , span : Span , annotation : TokenStream , annotated : TokenStream ,) -> Result < TokenStream , ErrorGuaranteed > { expand_ensures_tts (ecx , span , annotation , annotated) } }}}

macro_rules! expand_contract_clause_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_contract_clause in module {}", module_path!());
    };
}

mkfn!{
    expand_contract_clause_introspect!();
    # [doc = " Expand the function signature to include the contract clause."] # [doc = ""] # [doc = " The contracts clause will be injected before the function body and the optional where clause."] # [doc = " For that, we search for the body / where token, and invoke the `inject` callback to generate the"] # [doc = " contract clause in the right place."] # [doc = ""] fn expand_contract_clause (ecx : & mut ExtCtxt < '_ > , attr_span : Span , annotated : TokenStream , inject : impl FnOnce (& mut TokenStream) -> Result < () , ErrorGuaranteed > ,) -> Result < TokenStream , ErrorGuaranteed > { let mut new_tts = TokenStream :: default () ; let mut cursor = annotated . iter () ; let is_kw = | tt : & TokenTree , sym : Symbol | { if let TokenTree :: Token (token , _) = tt { token . is_ident_named (sym) } else { false } } ; if cursor . find (| tt | { new_tts . push_tree ((* tt) . clone ()) ; is_kw (tt , kw :: Fn) }) . is_none () { return Err (ecx . sess . dcx () . span_err (attr_span , "contract annotations can only be used on functions")) ; } let next_tt = loop { let Some (tt) = cursor . next () else { return Err (ecx . sess . dcx () . span_err (attr_span , "contract annotations is only supported in functions with bodies" ,)) ; } ; if cursor . peek () . is_none () { if let TokenTree :: Delimited (_ , _ , token :: Delimiter :: Brace , _) = tt { break tt ; } else { return Err (ecx . sess . dcx () . span_err (attr_span , "contract annotations is only supported in functions with bodies" ,)) ; } } if is_kw (tt , kw :: Where) { break tt ; } new_tts . push_tree (tt . clone ()) ; } ; inject (& mut new_tts) ? ; new_tts . push_tree (next_tt . clone ()) ; while let Some (tt) = cursor . next () { new_tts . push_tree (tt . clone ()) ; if cursor . peek () . is_none () && ! matches ! (tt , TokenTree :: Delimited (_ , _ , token :: Delimiter :: Brace , _)) { return Err (ecx . sess . dcx () . span_err (attr_span , "contract annotations is only supported in functions with bodies" ,)) ; } } Ok (new_tts) }
}

macro_rules! expand_requires_tts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_requires_tts in module {}", module_path!());
    };
}

mkfn!{
    expand_requires_tts_introspect!();
    fn expand_requires_tts (ecx : & mut ExtCtxt < '_ > , attr_span : Span , annotation : TokenStream , annotated : TokenStream ,) -> Result < TokenStream , ErrorGuaranteed > { let feature_span = ecx . with_def_site_ctxt (attr_span) ; expand_contract_clause (ecx , attr_span , annotated , | new_tts | { new_tts . push_tree (TokenTree :: Token (token :: Token :: from_ast_ident (Ident :: new (kw :: ContractRequires , feature_span)) , Spacing :: Joint ,)) ; new_tts . push_tree (TokenTree :: Token (token :: Token :: new (token :: TokenKind :: OrOr , attr_span) , Spacing :: Alone ,)) ; new_tts . push_tree (TokenTree :: Delimited (DelimSpan :: from_single (attr_span) , DelimSpacing :: new (Spacing :: JointHidden , Spacing :: JointHidden) , token :: Delimiter :: Parenthesis , annotation ,)) ; Ok (()) }) }
}

macro_rules! expand_ensures_tts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_ensures_tts in module {}", module_path!());
    };
}

mkfn!{
    expand_ensures_tts_introspect!();
    fn expand_ensures_tts (ecx : & mut ExtCtxt < '_ > , attr_span : Span , annotation : TokenStream , annotated : TokenStream ,) -> Result < TokenStream , ErrorGuaranteed > { let feature_span = ecx . with_def_site_ctxt (attr_span) ; expand_contract_clause (ecx , attr_span , annotated , | new_tts | { new_tts . push_tree (TokenTree :: Token (token :: Token :: from_ast_ident (Ident :: new (kw :: ContractEnsures , feature_span)) , Spacing :: Joint ,)) ; new_tts . push_tree (TokenTree :: Delimited (DelimSpan :: from_single (attr_span) , DelimSpacing :: new (Spacing :: JointHidden , Spacing :: JointHidden) , token :: Delimiter :: Parenthesis , annotation ,)) ; Ok (()) }) }
}