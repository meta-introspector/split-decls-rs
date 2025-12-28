macro_rules! deps {
    () => {
        ForLoopsOverFalliblesDiag!();
        LateContext!();
        ForLoopsOverFalliblesQuestionMark!();
        ForLoopsOverFalliblesLoopSub!();
        ForLoopsOverFalliblesSuggestion!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for ForLoopsOverFallibles { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let Some ((pat , arg)) = extract_for_loop (expr) else { return } ; let arg_span = arg . span . source_callsite () ; let ty = cx . typeck_results () . expr_ty (arg) ; let (adt , args , ref_mutability) = match ty . kind () { & ty :: Adt (adt , args) => (adt , args , None) , & ty :: Ref (_ , ty , mutability) => match ty . kind () { & ty :: Adt (adt , args) => (adt , args , Some (mutability)) , _ => return , } , _ => return , } ; let (article , ty , var) = match adt . did () { did if cx . tcx . is_diagnostic_item (sym :: Option , did) && ref_mutability . is_some () => { ("a" , "Option" , "Some") } did if cx . tcx . is_diagnostic_item (sym :: Option , did) => ("an" , "Option" , "Some") , did if cx . tcx . is_diagnostic_item (sym :: Result , did) => ("a" , "Result" , "Ok") , _ => return , } ; let ref_prefix = match ref_mutability { None => "" , Some (ref_mutability) => ref_mutability . ref_prefix_str () , } ; let sub = if let Some (recv) = extract_iterator_next_call (cx , arg) && let Ok (recv_snip) = cx . sess () . source_map () . span_to_snippet (recv . span) { ForLoopsOverFalliblesLoopSub :: RemoveNext { suggestion : recv . span . between (arg_span . shrink_to_hi ()) , recv_snip , } } else { ForLoopsOverFalliblesLoopSub :: UseWhileLet { start_span : expr . span . with_hi (pat . span . lo ()) , end_span : pat . span . between (arg_span) , var , } } ; let question_mark = suggest_question_mark (cx , adt , args , expr . span) . then (| | ForLoopsOverFalliblesQuestionMark { suggestion : arg_span . shrink_to_hi () }) ; let suggestion = ForLoopsOverFalliblesSuggestion { var , start_span : expr . span . with_hi (pat . span . lo ()) , end_span : pat . span . between (arg_span) , } ; cx . emit_span_lint (FOR_LOOPS_OVER_FALLIBLES , arg_span , ForLoopsOverFalliblesDiag { article , ref_prefix , ty , sub , question_mark , suggestion } ,) ; } }
    };
}

impl_209!()