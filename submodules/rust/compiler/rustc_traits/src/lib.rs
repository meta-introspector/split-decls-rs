mkmod!{codegen, { 
                getname!(codegen);
                getsrc!(codegen);
                getpath!(codegen);
                get_deps!(codegen);
                get_crates!(codegen);
                mkinclude!(codegen);
                 
            }}
mkmod!{coroutine_witnesses, { 
                getname!(coroutine_witnesses);
                getsrc!(coroutine_witnesses);
                getpath!(coroutine_witnesses);
                get_deps!(coroutine_witnesses);
                get_crates!(coroutine_witnesses);
                mkinclude!(coroutine_witnesses);
                 
            }}
mkmod!{dropck_outlives, { 
                getname!(dropck_outlives);
                getsrc!(dropck_outlives);
                getpath!(dropck_outlives);
                get_deps!(dropck_outlives);
                get_crates!(dropck_outlives);
                mkinclude!(dropck_outlives);
                 
            }}
mkmod!{evaluate_obligation, { 
                getname!(evaluate_obligation);
                getsrc!(evaluate_obligation);
                getpath!(evaluate_obligation);
                get_deps!(evaluate_obligation);
                get_crates!(evaluate_obligation);
                mkinclude!(evaluate_obligation);
                 
            }}
mkmod!{implied_outlives_bounds, { 
                getname!(implied_outlives_bounds);
                getsrc!(implied_outlives_bounds);
                getpath!(implied_outlives_bounds);
                get_deps!(implied_outlives_bounds);
                get_crates!(implied_outlives_bounds);
                mkinclude!(implied_outlives_bounds);
                 
            }}
mkmod!{normalize_erasing_regions, { 
                getname!(normalize_erasing_regions);
                getsrc!(normalize_erasing_regions);
                getpath!(normalize_erasing_regions);
                get_deps!(normalize_erasing_regions);
                get_crates!(normalize_erasing_regions);
                mkinclude!(normalize_erasing_regions);
                 
            }}
mkmod!{normalize_projection_ty, { 
                getname!(normalize_projection_ty);
                getsrc!(normalize_projection_ty);
                getpath!(normalize_projection_ty);
                get_deps!(normalize_projection_ty);
                get_crates!(normalize_projection_ty);
                mkinclude!(normalize_projection_ty);
                 
            }}
mkmod!{type_op, { 
                getname!(type_op);
                getsrc!(type_op);
                getpath!(type_op);
                get_deps!(type_op);
                get_crates!(type_op);
                mkinclude!(type_op);
                 
            }}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{pub use rustc_trait_selection :: traits :: query :: type_op :: ascribe_user_type :: type_op_ascribe_user_type_with_span ;}
mkuse!{pub use type_op :: type_op_prove_predicate_with_cause ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (p : & mut Providers) { dropck_outlives :: provide (p) ; evaluate_obligation :: provide (p) ; implied_outlives_bounds :: provide (p) ; normalize_projection_ty :: provide (p) ; normalize_erasing_regions :: provide (p) ; type_op :: provide (p) ; p . codegen_select_candidate = codegen :: codegen_select_candidate ; p . coroutine_hidden_types = coroutine_witnesses :: coroutine_hidden_types ; }
}