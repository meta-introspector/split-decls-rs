mkuse!{use rustc_middle :: query :: Providers ;}
mkmod!{abi, { 
                getname!(abi);
                getsrc!(abi);
                getpath!(abi);
                get_deps!(abi);
                get_crates!(abi);
                mkinclude!(abi);
                 
            }}
mkmod!{assoc, { 
                getname!(assoc);
                getsrc!(assoc);
                getpath!(assoc);
                get_deps!(assoc);
                get_crates!(assoc);
                mkinclude!(assoc);
                 
            }}
mkmod!{common_traits, { 
                getname!(common_traits);
                getsrc!(common_traits);
                getpath!(common_traits);
                get_deps!(common_traits);
                get_crates!(common_traits);
                mkinclude!(common_traits);
                 
            }}
mkmod!{consts, { 
                getname!(consts);
                getsrc!(consts);
                getpath!(consts);
                get_deps!(consts);
                get_crates!(consts);
                mkinclude!(consts);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{implied_bounds, { 
                getname!(implied_bounds);
                getsrc!(implied_bounds);
                getpath!(implied_bounds);
                get_deps!(implied_bounds);
                get_crates!(implied_bounds);
                mkinclude!(implied_bounds);
                 
            }}
mkmod!{instance, { 
                getname!(instance);
                getsrc!(instance);
                getpath!(instance);
                get_deps!(instance);
                get_crates!(instance);
                mkinclude!(instance);
                 
            }}
mkmod!{layout, { 
                getname!(layout);
                getsrc!(layout);
                getpath!(layout);
                get_deps!(layout);
                get_crates!(layout);
                mkinclude!(layout);
                 
            }}
mkmod!{needs_drop, { 
                getname!(needs_drop);
                getsrc!(needs_drop);
                getpath!(needs_drop);
                get_deps!(needs_drop);
                get_crates!(needs_drop);
                mkinclude!(needs_drop);
                 
            }}
mkmod!{nested_bodies, { 
                getname!(nested_bodies);
                getsrc!(nested_bodies);
                getpath!(nested_bodies);
                get_deps!(nested_bodies);
                get_crates!(nested_bodies);
                mkinclude!(nested_bodies);
                 
            }}
mkmod!{opaque_types, { 
                getname!(opaque_types);
                getsrc!(opaque_types);
                getpath!(opaque_types);
                get_deps!(opaque_types);
                get_crates!(opaque_types);
                mkinclude!(opaque_types);
                 
            }}
mkmod!{representability, { 
                getname!(representability);
                getsrc!(representability);
                getpath!(representability);
                get_deps!(representability);
                get_crates!(representability);
                mkinclude!(representability);
                 
            }}
mkmod!{sig_types, { 
                getname!(sig_types);
                getsrc!(sig_types);
                getpath!(sig_types);
                get_deps!(sig_types);
                get_crates!(sig_types);
                mkinclude!(sig_types);
                 
            }}
mkmod!{structural_match, { 
                getname!(structural_match);
                getsrc!(structural_match);
                getpath!(structural_match);
                get_deps!(structural_match);
                get_crates!(structural_match);
                mkinclude!(structural_match);
                 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { abi :: provide (providers) ; assoc :: provide (providers) ; common_traits :: provide (providers) ; consts :: provide (providers) ; implied_bounds :: provide (providers) ; layout :: provide (providers) ; needs_drop :: provide (providers) ; opaque_types :: provide (providers) ; representability :: provide (providers) ; ty :: provide (providers) ; instance :: provide (providers) ; structural_match :: provide (providers) ; nested_bodies :: provide (providers) ; }
}