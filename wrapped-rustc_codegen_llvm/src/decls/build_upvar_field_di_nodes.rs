macro_rules! deps {
    () => {
        CodegenCx!();
        SmallVec!();
    };
}

macro_rules! build_upvar_field_di_nodes {
    () => {
        deps!();
        # [doc = " Builds the DW_TAG_member debuginfo nodes for the upvars of a closure or coroutine."] # [doc = " For a coroutine, this will handle upvars shared by all states."] fn build_upvar_field_di_nodes < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , closure_or_coroutine_ty : Ty < 'tcx > , closure_or_coroutine_di_node : & 'll DIType ,) -> SmallVec < & 'll DIType > { let (& def_id , up_var_tys) = match closure_or_coroutine_ty . kind () { ty :: Coroutine (def_id , args) => (def_id , args . as_coroutine () . prefix_tys ()) , ty :: Closure (def_id , args) => (def_id , args . as_closure () . upvar_tys ()) , ty :: CoroutineClosure (def_id , args) => (def_id , args . as_coroutine_closure () . upvar_tys ()) , _ => { bug ! ("build_upvar_field_di_nodes() called with non-closure-or-coroutine-type: {:?}" , closure_or_coroutine_ty) } } ; assert ! (up_var_tys . iter () . all (| t | t == cx . tcx . normalize_erasing_regions (cx . typing_env () , t))) ; let capture_names = cx . tcx . closure_saved_names_of_captured_variables (def_id) ; let layout = cx . layout_of (closure_or_coroutine_ty) ; up_var_tys . into_iter () . zip (capture_names . iter ()) . enumerate () . map (| (index , (up_var_ty , capture_name)) | { build_field_di_node (cx , closure_or_coroutine_di_node , capture_name . as_str () , cx . layout_of (up_var_ty) , layout . fields . offset (index) , DIFlags :: FlagZero , type_di_node (cx , up_var_ty) , None ,) }) . collect () }
    };
}

build_upvar_field_di_nodes!();