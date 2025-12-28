macro_rules! deps {
    () => {
        TopInfo!();
        FnCtxt!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { fn pattern_cause (& self , ti : & TopInfo < 'tcx > , cause_span : Span) -> ObligationCause < 'tcx > { let origin_expr_info = ti . origin_expr . map (| mut cur_expr | { let mut count = 0 ; while let ExprKind :: AddrOf (.. , inner) = & cur_expr . kind { cur_expr = inner ; count += 1 ; } PatternOriginExpr { peeled_span : cur_expr . span , peeled_count : count , peeled_prefix_suggestion_parentheses : expr_needs_parens (cur_expr) , } }) ; let code = ObligationCauseCode :: Pattern { span : ti . span , root_ty : ti . expected , origin_expr : origin_expr_info , } ; self . cause (cause_span , code) } fn demand_eqtype_pat_diag (& 'a self , cause_span : Span , expected : Ty < 'tcx > , actual : Ty < 'tcx > , ti : & TopInfo < 'tcx > ,) -> Result < () , Diag < 'a > > { self . demand_eqtype_with_origin (& self . pattern_cause (ti , cause_span) , expected , actual) . map_err (| mut diag | { if let Some (expr) = ti . origin_expr { self . suggest_fn_call (& mut diag , expr , expected , | output | { self . can_eq (self . param_env , output , actual) }) ; } diag }) } fn demand_eqtype_pat (& self , cause_span : Span , expected : Ty < 'tcx > , actual : Ty < 'tcx > , ti : & TopInfo < 'tcx > ,) -> Result < () , ErrorGuaranteed > { self . demand_eqtype_pat_diag (cause_span , expected , actual , ti) . map_err (| err | err . emit ()) } }
    };
}

impl_317!();