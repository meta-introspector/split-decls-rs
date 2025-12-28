macro_rules! deps {
    () => {
        CheckLoopVisitor!();
        BreakContextKind!();
        BreakInsideCoroutine!();
        BlockInfo!();
        Context!();
        UnlabeledInLabeledBlock!();
        BreakInsideClosure!();
        OutsideLoopSuggestion!();
        OutsideLoop!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < 'hir > CheckLoopVisitor < 'hir > { fn with_context < F > (& mut self , cx : Context , f : F) where F : FnOnce (& mut CheckLoopVisitor < 'hir >) , { self . cx_stack . push (cx) ; f (self) ; self . cx_stack . pop () ; } fn require_break_cx (& mut self , br_cx_kind : BreakContextKind , span : Span , break_span : Span , cx_pos : usize ,) { match self . cx_stack [cx_pos] { LabeledBlock | Loop (_) | LoopMatch { .. } => { } Closure (closure_span) => { self . tcx . dcx () . emit_err (BreakInsideClosure { span , closure_span , name : & br_cx_kind . to_string () , }) ; } Coroutine { coroutine_span , kind , source } => { let kind = match kind { hir :: CoroutineDesugaring :: Async => "async" , hir :: CoroutineDesugaring :: Gen => "gen" , hir :: CoroutineDesugaring :: AsyncGen => "async gen" , } ; let source = match source { hir :: CoroutineSource :: Block => "block" , hir :: CoroutineSource :: Closure => "closure" , hir :: CoroutineSource :: Fn => "function" , } ; self . tcx . dcx () . emit_err (BreakInsideCoroutine { span , coroutine_span , name : & br_cx_kind . to_string () , kind , source , }) ; } UnlabeledBlock (block_span) if br_cx_kind == BreakContextKind :: Break && block_span . eq_ctxt (break_span) => { let block = self . block_breaks . entry (block_span) . or_insert_with (| | BlockInfo { name : br_cx_kind . to_string () , spans : vec ! [] , suggs : vec ! [] , }) ; block . spans . push (span) ; block . suggs . push (break_span) ; } UnlabeledIfBlock (_) if br_cx_kind == BreakContextKind :: Break => { self . require_break_cx (br_cx_kind , span , break_span , cx_pos - 1) ; } Normal | AnonConst | Fn | UnlabeledBlock (_) | UnlabeledIfBlock (_) | ConstBlock => { self . tcx . dcx () . emit_err (OutsideLoop { spans : vec ! [span] , name : & br_cx_kind . to_string () , is_break : br_cx_kind == BreakContextKind :: Break , suggestion : None , }) ; } } } fn require_label_in_labeled_block (& self , span : Span , label : & Destination , cf_type : & str ,) -> bool { if ! span . is_desugaring (DesugaringKind :: QuestionMark) && self . cx_stack . last () == Some (& LabeledBlock) && label . label . is_none () { self . tcx . dcx () . emit_err (UnlabeledInLabeledBlock { span , cf_type }) ; return true ; } false } fn report_outside_loop_error (& self) { for (s , block) in & self . block_breaks { self . tcx . dcx () . emit_err (OutsideLoop { spans : block . spans . clone () , name : & block . name , is_break : true , suggestion : Some (OutsideLoopSuggestion { block_span : * s , break_spans : block . suggs . clone () , }) , }) ; } } # [doc = " Is this a loop annotated with `#[loop_match]` that looks syntactically sound?"] fn is_loop_match (& self , e : & 'hir hir :: Expr < 'hir > , body : & 'hir hir :: Block < 'hir > ,) -> Option < Destination > { if ! find_attr ! (self . tcx . hir_attrs (e . hir_id) , AttributeKind :: LoopMatch (_)) { return None ; } let loop_body_expr = match body . stmts { [] => match body . expr { Some (expr) => expr , None => return None , } , [single] if body . expr . is_none () => match single . kind { hir :: StmtKind :: Expr (expr) | hir :: StmtKind :: Semi (expr) => expr , _ => return None , } , [..] => return None , } ; let hir :: ExprKind :: Assign (_ , rhs_expr , _) = loop_body_expr . kind else { return None } ; let hir :: ExprKind :: Block (block , label) = rhs_expr . kind else { return None } ; Some (Destination { label , target_id : Ok (block . hir_id) }) } }
    };
}

impl_241!();