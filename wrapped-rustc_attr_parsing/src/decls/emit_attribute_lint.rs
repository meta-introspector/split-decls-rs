macro_rules! deps {
    () => {
        IllFormedAttributeInput!();
        InvalidTargetLint!();
        UnusedDuplicate!();
        InvalidAttrStyle!();
        EmptyAttributeList!();
        InvalidTarget!();
    };
}

macro_rules! emit_attribute_lint {
    () => {
        deps!();
        pub fn emit_attribute_lint < L : LintEmitter > (lint : & AttributeLint < L :: Id > , lint_emitter : L) { let AttributeLint { id , span , kind } = lint ; match kind { & AttributeLintKind :: UnusedDuplicate { this , other , warning } => lint_emitter . emit_node_span_lint (rustc_session :: lint :: builtin :: UNUSED_ATTRIBUTES , * id , * span , session_diagnostics :: UnusedDuplicate { this , other , warning } ,) , AttributeLintKind :: IllFormedAttributeInput { suggestions } => { lint_emitter . emit_node_span_lint (rustc_session :: lint :: builtin :: ILL_FORMED_ATTRIBUTE_INPUT , * id , * span , session_diagnostics :: IllFormedAttributeInput { num_suggestions : suggestions . len () , suggestions : DiagArgValue :: StrListSepByAnd (suggestions . into_iter () . map (| s | format ! ("`{s}`") . into ()) . collect () ,) , } ,) ; } AttributeLintKind :: EmptyAttribute { first_span } => lint_emitter . emit_node_span_lint (rustc_session :: lint :: builtin :: UNUSED_ATTRIBUTES , * id , * first_span , session_diagnostics :: EmptyAttributeList { attr_span : * first_span } ,) , AttributeLintKind :: InvalidTarget { name , target , applied , only } => lint_emitter . emit_node_span_lint (if name . segments [0] . name == sym :: deprecated && ! [Target :: Closure , Target :: Expression , Target :: Statement , Target :: Arm , Target :: MacroCall ,] . contains (target) { rustc_session :: lint :: builtin :: USELESS_DEPRECATED } else { rustc_session :: lint :: builtin :: UNUSED_ATTRIBUTES } , * id , * span , session_diagnostics :: InvalidTargetLint { name : name . clone () , target : target . plural_name () , applied : DiagArgValue :: StrListSepByAnd (applied . into_iter () . map (| i | Cow :: Owned (i . to_string ())) . collect () ,) , only , attr_span : * span , } ,) , & AttributeLintKind :: InvalidStyle { ref name , is_used_as_inner , target , target_span } => { lint_emitter . emit_node_span_lint (rustc_session :: lint :: builtin :: UNUSED_ATTRIBUTES , * id , * span , session_diagnostics :: InvalidAttrStyle { name : name . clone () , is_used_as_inner , target_span : (! is_used_as_inner) . then_some (target_span) , target , } ,) } } }
    };
}

emit_attribute_lint!()