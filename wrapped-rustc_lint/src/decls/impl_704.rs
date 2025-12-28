macro_rules! deps {
    () => {
        NonUpperCaseGlobalSubTool!();
        LateContext!();
        NonUpperCaseGlobal!();
        NonUpperCaseGlobalSub!();
    };
}

macro_rules! impl_704 {
    () => {
        deps!();
        impl NonUpperCaseGlobals { fn check_upper_case (cx : & LateContext < '_ > , sort : & str , did : Option < LocalDefId > , ident : & Ident) { let name = ident . name . as_str () ; if name . chars () . any (| c | c . is_lowercase ()) { let uc = NonSnakeCase :: to_snake_case (name) . to_uppercase () ; let can_change_usages = if let Some (did) = did { ! cx . tcx . effective_visibilities (()) . is_exported (did) } else { false } ; let sub = if * name != uc { NonUpperCaseGlobalSub :: Suggestion { span : ident . span , replace : uc . clone () , applicability : if can_change_usages { Applicability :: MachineApplicable } else { Applicability :: MaybeIncorrect } , } } else { NonUpperCaseGlobalSub :: Label { span : ident . span } } ; struct UsageCollector < 'a , 'tcx > { cx : & 'tcx LateContext < 'a > , did : DefId , collected : Vec < Span > , } impl < 'v , 'tcx > Visitor < 'v > for UsageCollector < 'v , 'tcx > { type NestedFilter = All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } fn visit_path (& mut self , path : & rustc_hir :: Path < 'v > , _id : rustc_hir :: HirId ,) -> Self :: Result { if let Some (final_seg) = path . segments . last () && final_seg . res . opt_def_id () == Some (self . did) { self . collected . push (final_seg . ident . span) ; } } } cx . emit_span_lint_lazy (NON_UPPER_CASE_GLOBALS , ident . span , | | { let usages = if can_change_usages && * name != uc && let Some (did) = did { let mut usage_collector = UsageCollector { cx , did : did . to_def_id () , collected : Vec :: new () } ; cx . tcx . hir_walk_toplevel_module (& mut usage_collector) ; usage_collector . collected . into_iter () . map (| span | NonUpperCaseGlobalSubTool { span , replace : uc . clone () }) . collect () } else { vec ! [] } ; NonUpperCaseGlobal { sort , name , sub , usages } }) ; } } }
    };
}

impl_704!()