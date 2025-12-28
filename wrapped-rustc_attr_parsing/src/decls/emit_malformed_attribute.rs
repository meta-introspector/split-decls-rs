macro_rules! deps {
    () => {
        IllFormedAttributeInput!();
    };
}

macro_rules! emit_malformed_attribute {
    () => {
        deps!();
        fn emit_malformed_attribute (psess : & ParseSess , style : ast :: AttrStyle , span : Span , name : Symbol , template : AttributeTemplate ,) { let should_warn = | name | matches ! (name , sym :: doc | sym :: link | sym :: test | sym :: bench) ; let error_msg = format ! ("malformed `{name}` attribute input") ; let mut suggestions = vec ! [] ; let inner = if style == ast :: AttrStyle :: Inner { "!" } else { "" } ; if template . word { suggestions . push (format ! ("#{inner}[{name}]")) ; } if let Some (descr) = template . list { for descr in descr { suggestions . push (format ! ("#{inner}[{name}({descr})]")) ; } } suggestions . extend (template . one_of . iter () . map (| & word | format ! ("#{inner}[{name}({word})]"))) ; if let Some (descr) = template . name_value_str { for descr in descr { suggestions . push (format ! ("#{inner}[{name} = \"{descr}\"]")) ; } } if should_warn (name) { psess . buffer_lint (ILL_FORMED_ATTRIBUTE_INPUT , span , ast :: CRATE_NODE_ID , BuiltinLintDiag :: IllFormedAttributeInput { suggestions : suggestions . clone () , docs : template . docs , } ,) ; } else { suggestions . sort () ; let mut err = psess . dcx () . struct_span_err (span , error_msg) . with_span_suggestions (span , if suggestions . len () == 1 { "must be of the form" } else { "the following are the possible correct uses" } , suggestions , Applicability :: HasPlaceholders ,) ; if let Some (link) = template . docs { err . note (format ! ("for more information, visit <{link}>")) ; } err . emit () ; } }
    };
}

emit_malformed_attribute!();