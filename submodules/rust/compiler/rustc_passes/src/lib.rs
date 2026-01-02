mkuse!{use rustc_middle :: util :: Providers ;}
mkmod!{abi_test, { 
                getname!(abi_test);
                getsrc!(abi_test);
                getpath!(abi_test);
                get_deps!(abi_test);
                get_crates!(abi_test);
                mkinclude!(abi_test);
                 
            }}
mkmod!{check_attr, { 
                getname!(check_attr);
                getsrc!(check_attr);
                getpath!(check_attr);
                get_deps!(check_attr);
                get_crates!(check_attr);
                mkinclude!(check_attr);
                 
            }}
mkmod!{check_export, { 
                getname!(check_export);
                getsrc!(check_export);
                getpath!(check_export);
                get_deps!(check_export);
                get_crates!(check_export);
                mkinclude!(check_export);
                 
            }}
mkmod!{dead, { 
                getname!(dead);
                getsrc!(dead);
                getpath!(dead);
                get_deps!(dead);
                get_crates!(dead);
                mkinclude!(dead);
                 
            }}
mkmod!{debugger_visualizer, { 
                getname!(debugger_visualizer);
                getsrc!(debugger_visualizer);
                getpath!(debugger_visualizer);
                get_deps!(debugger_visualizer);
                get_crates!(debugger_visualizer);
                mkinclude!(debugger_visualizer);
                 
            }}
mkmod!{diagnostic_items, { 
                getname!(diagnostic_items);
                getsrc!(diagnostic_items);
                getpath!(diagnostic_items);
                get_deps!(diagnostic_items);
                get_crates!(diagnostic_items);
                mkinclude!(diagnostic_items);
                 
            }}
mkmod!{entry, { 
                getname!(entry);
                getsrc!(entry);
                getpath!(entry);
                get_deps!(entry);
                get_crates!(entry);
                mkinclude!(entry);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{hir_id_validator, { 
                getname!(hir_id_validator);
                getsrc!(hir_id_validator);
                getpath!(hir_id_validator);
                get_deps!(hir_id_validator);
                get_crates!(hir_id_validator);
                mkinclude!(hir_id_validator);
                 
            }}
mkmod!{input_stats, { 
                getname!(input_stats);
                getsrc!(input_stats);
                getpath!(input_stats);
                get_deps!(input_stats);
                get_crates!(input_stats);
                mkinclude!(input_stats);
                 
            }}
mkmod!{lang_items, { 
                getname!(lang_items);
                getsrc!(lang_items);
                getpath!(lang_items);
                get_deps!(lang_items);
                get_crates!(lang_items);
                mkinclude!(lang_items);
                 
            }}
mkmod!{layout_test, { 
                getname!(layout_test);
                getsrc!(layout_test);
                getpath!(layout_test);
                get_deps!(layout_test);
                get_crates!(layout_test);
                mkinclude!(layout_test);
                 
            }}
mkmod!{lib_features, { 
                getname!(lib_features);
                getsrc!(lib_features);
                getpath!(lib_features);
                get_deps!(lib_features);
                get_crates!(lib_features);
                mkinclude!(lib_features);
                 
            }}
mkmod!{liveness, { 
                getname!(liveness);
                getsrc!(liveness);
                getpath!(liveness);
                get_deps!(liveness);
                get_crates!(liveness);
                mkinclude!(liveness);
                 
            }}
mkmod!{reachable, { 
                getname!(reachable);
                getsrc!(reachable);
                getpath!(reachable);
                get_deps!(reachable);
                get_crates!(reachable);
                mkinclude!(reachable);
                 
            }}
mkmod!{stability, { 
                getname!(stability);
                getsrc!(stability);
                getpath!(stability);
                get_deps!(stability);
                get_crates!(stability);
                mkinclude!(stability);
                 
            }}
mkmod!{upvars, { 
                getname!(upvars);
                getsrc!(upvars);
                getpath!(upvars);
                get_deps!(upvars);
                get_crates!(upvars);
                mkinclude!(upvars);
                 
            }}
mkmod!{weak_lang_items, { 
                getname!(weak_lang_items);
                getsrc!(weak_lang_items);
                getpath!(weak_lang_items);
                get_deps!(weak_lang_items);
                get_crates!(weak_lang_items);
                mkinclude!(weak_lang_items);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { check_attr :: provide (providers) ; dead :: provide (providers) ; debugger_visualizer :: provide (providers) ; diagnostic_items :: provide (providers) ; entry :: provide (providers) ; lang_items :: provide (providers) ; lib_features :: provide (providers) ; liveness :: provide (providers) ; reachable :: provide (providers) ; stability :: provide (providers) ; upvars :: provide (providers) ; check_export :: provide (providers) ; }
}