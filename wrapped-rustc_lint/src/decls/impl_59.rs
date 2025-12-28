macro_rules! deps {
    () => {
        LateContext!();
        BuiltinUngatedAsyncFnTrackCaller!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for UngatedAsyncFnTrackCaller { fn check_fn (& mut self , cx : & LateContext < '_ > , fn_kind : HirFnKind < '_ > , _ : & 'tcx FnDecl < '_ > , _ : & 'tcx Body < '_ > , span : Span , def_id : LocalDefId ,) { if fn_kind . asyncness () . is_async () && ! cx . tcx . features () . async_fn_track_caller () && let Some (attr_span) = find_attr ! (cx . tcx . get_all_attrs (def_id) , AttributeKind :: TrackCaller (span) => * span) { cx . emit_span_lint (UNGATED_ASYNC_FN_TRACK_CALLER , attr_span , BuiltinUngatedAsyncFnTrackCaller { label : span , session : & cx . tcx . sess } ,) ; } } }
    };
}

impl_59!()