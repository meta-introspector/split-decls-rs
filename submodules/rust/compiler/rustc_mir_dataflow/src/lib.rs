mkuse!{use rustc_middle :: ty ;}
mkuse!{pub use self :: drop_flag_effects :: { DropFlagState , drop_flag_effects_for_function_entry , drop_flag_effects_for_location , move_path_children_matching , on_all_children_bits , on_lookup_result_bits , } ;}
mkuse!{pub use self :: framework :: { Analysis , Backward , Direction , Forward , GenKill , JoinSemiLattice , MaybeReachable , Results , ResultsCursor , ResultsVisitor , fmt , graphviz , lattice , visit_reachable_results , visit_results , } ;}
mkuse!{use self :: move_paths :: MoveData ;}
mkmod!{debuginfo, { 
                getname!(debuginfo);
                getsrc!(debuginfo);
                getpath!(debuginfo);
                get_deps!(debuginfo);
                get_crates!(debuginfo);
                mkinclude!(debuginfo);
                 
            }}
mkmod!{drop_flag_effects, { 
                getname!(drop_flag_effects);
                getsrc!(drop_flag_effects);
                getpath!(drop_flag_effects);
                get_deps!(drop_flag_effects);
                get_crates!(drop_flag_effects);
                mkinclude!(drop_flag_effects);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{framework, { 
                getname!(framework);
                getsrc!(framework);
                getpath!(framework);
                get_deps!(framework);
                get_crates!(framework);
                mkinclude!(framework);
                 
            }}
mkmod!{impls, { 
                getname!(impls);
                getsrc!(impls);
                getpath!(impls);
                get_deps!(impls);
                get_crates!(impls);
                mkinclude!(impls);
                 
            }}
mkmod!{move_paths, { 
                getname!(move_paths);
                getsrc!(move_paths);
                getpath!(move_paths);
                get_deps!(move_paths);
                get_crates!(move_paths);
                mkinclude!(move_paths);
                 
            }}
mkmod!{points, { 
                getname!(points);
                getsrc!(points);
                getpath!(points);
                get_deps!(points);
                get_crates!(points);
                mkinclude!(points);
                 
            }}
mkmod!{rustc_peek, { 
                getname!(rustc_peek);
                getsrc!(rustc_peek);
                getpath!(rustc_peek);
                get_deps!(rustc_peek);
                get_crates!(rustc_peek);
                mkinclude!(rustc_peek);
                 
            }}
mkmod!{un_derefer, { 
                getname!(un_derefer);
                getsrc!(un_derefer);
                getpath!(un_derefer);
                get_deps!(un_derefer);
                get_crates!(un_derefer);
                mkinclude!(un_derefer);
                 
            }}
mkmod!{value_analysis, { 
                getname!(value_analysis);
                getsrc!(value_analysis);
                getpath!(value_analysis);
                get_deps!(value_analysis);
                get_crates!(value_analysis);
                mkinclude!(value_analysis);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}
mkitem!{mkstruct!{pub struct MoveDataTypingEnv < 'tcx > { pub move_data : MoveData < 'tcx > , pub typing_env : ty :: TypingEnv < 'tcx > , }}}