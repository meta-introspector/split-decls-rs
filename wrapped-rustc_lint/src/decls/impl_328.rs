macro_rules! deps {
    () => {
        NonBindingLetSub!();
        NonBindingLet!();
        LateContext!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for LetUnderscore { fn check_local (& mut self , cx : & LateContext < '_ > , local : & hir :: LetStmt < '_ >) { if matches ! (local . source , rustc_hir :: LocalSource :: AsyncFn) { return ; } let mut top_level = true ; local . pat . walk_always (| pat | { let is_top_level = top_level ; top_level = false ; if ! matches ! (pat . kind , hir :: PatKind :: Wild) { return ; } let ty = cx . typeck_results () . pat_ty (pat) ; if ! ty . needs_drop (cx . tcx , cx . typing_env ()) { return ; } let potential_lock_type = match ty . kind () { ty :: Adt (adt , args) if cx . tcx . is_diagnostic_item (sym :: Result , adt . did ()) => { args . type_at (0) } _ => ty , } ; let is_sync_lock = match potential_lock_type . kind () { ty :: Adt (adt , _) => SYNC_GUARD_SYMBOLS . iter () . any (| guard_symbol | cx . tcx . is_diagnostic_item (* guard_symbol , adt . did ())) , _ => false , } ; let can_use_init = is_top_level . then_some (local . init) . flatten () ; let sub = NonBindingLetSub { suggestion : pat . span , drop_fn_start_end : can_use_init . map (| init | (local . span . until (init . span) , init . span . shrink_to_hi ())) , is_assign_desugar : matches ! (local . source , rustc_hir :: LocalSource :: AssignDesugar (_)) , } ; if is_sync_lock { let span = MultiSpan :: from_span (pat . span) ; cx . emit_span_lint (LET_UNDERSCORE_LOCK , span , NonBindingLet :: SyncLock { sub , pat : pat . span } ,) ; } else if can_use_init . is_some () { cx . emit_span_lint (LET_UNDERSCORE_DROP , local . span , NonBindingLet :: DropType { sub }) ; } }) ; } }
    };
}

impl_328!()