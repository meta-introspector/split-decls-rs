mkmod!{check_consts, { 
                getname!(check_consts);
                getsrc!(check_consts);
                getpath!(check_consts);
                get_deps!(check_consts);
                get_crates!(check_consts);
                mkinclude!(check_consts);
                 
            }}
mkmod!{const_eval, { 
                getname!(const_eval);
                getsrc!(const_eval);
                getpath!(const_eval);
                get_deps!(const_eval);
                get_crates!(const_eval);
                mkinclude!(const_eval);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{interpret, { 
                getname!(interpret);
                getsrc!(interpret);
                getpath!(interpret);
                get_deps!(interpret);
                get_crates!(interpret);
                mkinclude!(interpret);
                 
            }}
mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                 
            }}
mkuse!{use std :: sync :: atomic :: AtomicBool ;}
mkuse!{use rustc_middle :: ty ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{pub use self :: errors :: ReportErrorExt ;}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { const_eval :: provide (providers) ; providers . tag_for_variant = const_eval :: tag_for_variant_provider ; providers . eval_to_const_value_raw = const_eval :: eval_to_const_value_raw_provider ; providers . eval_to_allocation_raw = const_eval :: eval_to_allocation_raw_provider ; providers . eval_static_initializer = const_eval :: eval_static_initializer_provider ; providers . hooks . const_caller_location = util :: caller_location :: const_caller_location_provider ; providers . eval_to_valtree = | tcx , ty :: PseudoCanonicalInput { typing_env , value } | { const_eval :: eval_to_valtree (tcx , typing_env , value) } ; providers . hooks . try_destructure_mir_constant_for_user_output = const_eval :: try_destructure_mir_constant_for_user_output ; providers . valtree_to_const_val = | tcx , cv | const_eval :: valtree_to_const_value (tcx , ty :: TypingEnv :: fully_monomorphized () , cv) ; providers . check_validity_requirement = | tcx , (init_kind , param_env_and_ty) | { util :: check_validity_requirement (tcx , init_kind , param_env_and_ty) } ; providers . hooks . validate_scalar_in_layout = | tcx , scalar , layout | util :: validate_scalar_in_layout (tcx , scalar , layout) ; }
}
mkitem!{# [doc = " `rustc_driver::main` installs a handler that will set this to `true` if"] # [doc = " the compiler has been sent a request to shut down, such as by a Ctrl-C."] # [doc = " This static lives here because it is only read by the interpreter."] pub static CTRL_C_RECEIVED : AtomicBool = AtomicBool :: new (false) ;}