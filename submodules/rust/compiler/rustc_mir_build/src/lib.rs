mkmod!{builder, { 
                getname!(builder);
                getsrc!(builder);
                getpath!(builder);
                get_deps!(builder);
                get_crates!(builder);
                mkinclude!(builder);
                 
            }}
mkmod!{check_tail_calls, { 
                getname!(check_tail_calls);
                getsrc!(check_tail_calls);
                getpath!(check_tail_calls);
                get_deps!(check_tail_calls);
                get_crates!(check_tail_calls);
                mkinclude!(check_tail_calls);
                 
            }}
mkmod!{check_unsafety, { 
                getname!(check_unsafety);
                getsrc!(check_unsafety);
                getpath!(check_unsafety);
                get_deps!(check_unsafety);
                get_crates!(check_unsafety);
                mkinclude!(check_unsafety);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{thir, { 
                getname!(thir);
                getsrc!(thir);
                getpath!(thir);
                get_deps!(thir);
                get_crates!(thir);
                mkinclude!(thir);
                 
            }}
mkuse!{use rustc_middle :: util :: Providers ;}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { providers . check_match = thir :: pattern :: check_match ; providers . lit_to_const = thir :: constant :: lit_to_const ; providers . closure_saved_names_of_captured_variables = builder :: closure_saved_names_of_captured_variables ; providers . check_unsafety = check_unsafety :: check_unsafety ; providers . check_tail_calls = check_tail_calls :: check_tail_calls ; providers . thir_body = thir :: cx :: thir_body ; }
}