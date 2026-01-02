mkmod!{assert_dep_graph, { 
                getname!(assert_dep_graph);
                getsrc!(assert_dep_graph);
                getpath!(assert_dep_graph);
                get_deps!(assert_dep_graph);
                get_crates!(assert_dep_graph);
                mkinclude!(assert_dep_graph);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{persist, { 
                getname!(persist);
                getsrc!(persist);
                getpath!(persist);
                get_deps!(persist);
                get_crates!(persist);
                mkinclude!(persist);
                 
            }}
mkuse!{pub use persist :: { LoadResult , copy_cgu_workproduct_to_incr_comp_cache_dir , finalize_session_directory , in_incr_comp_dir , in_incr_comp_dir_sess , load_query_result_cache , save_work_product_index , setup_dep_graph , } ;}
mkuse!{use rustc_middle :: util :: Providers ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    # [allow (missing_docs)] pub fn provide (providers : & mut Providers) { providers . hooks . save_dep_graph = | tcx | tcx . sess . time ("serialize_dep_graph" , | | persist :: save_dep_graph (tcx)) ; }
}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}