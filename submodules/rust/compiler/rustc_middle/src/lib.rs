mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{arena, { 
                getname!(arena);
                getsrc!(arena);
                getpath!(arena);
                get_deps!(arena);
                get_crates!(arena);
                mkinclude!(arena);
                 
            }}
mkmod!{error, { 
                getname!(error);
                getsrc!(error);
                getpath!(error);
                get_deps!(error);
                get_crates!(error);
                mkinclude!(error);
                 
            }}
mkmod!{hir, { 
                getname!(hir);
                getsrc!(hir);
                getpath!(hir);
                get_deps!(hir);
                get_crates!(hir);
                mkinclude!(hir);
                 
            }}
mkmod!{hooks, { 
                getname!(hooks);
                getsrc!(hooks);
                getpath!(hooks);
                get_deps!(hooks);
                get_crates!(hooks);
                mkinclude!(hooks);
                 
            }}
mkmod!{infer, { 
                getname!(infer);
                getsrc!(infer);
                getpath!(infer);
                get_deps!(infer);
                get_crates!(infer);
                mkinclude!(infer);
                 
            }}
mkmod!{lint, { 
                getname!(lint);
                getsrc!(lint);
                getpath!(lint);
                get_deps!(lint);
                get_crates!(lint);
                mkinclude!(lint);
                 
            }}
mkmod!{metadata, { 
                getname!(metadata);
                getsrc!(metadata);
                getpath!(metadata);
                get_deps!(metadata);
                get_crates!(metadata);
                mkinclude!(metadata);
                 
            }}
mkmod!{middle, { 
                getname!(middle);
                getsrc!(middle);
                getpath!(middle);
                get_deps!(middle);
                get_crates!(middle);
                mkinclude!(middle);
                 
            }}
mkmod!{mir, { 
                getname!(mir);
                getsrc!(mir);
                getpath!(mir);
                get_deps!(mir);
                get_crates!(mir);
                mkinclude!(mir);
                 
            }}
mkmod!{thir, { 
                getname!(thir);
                getsrc!(thir);
                getpath!(thir);
                get_deps!(thir);
                get_crates!(thir);
                mkinclude!(thir);
                 
            }}
mkmod!{traits, { 
                getname!(traits);
                getsrc!(traits);
                getpath!(traits);
                get_deps!(traits);
                get_crates!(traits);
                mkinclude!(traits);
                 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                 
            }}
mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                 
            }}
mkmod!{values, { 
                getname!(values);
                getsrc!(values);
                getpath!(values);
                get_deps!(values);
                get_crates!(values);
                mkinclude!(values);
                 
            }}
mkmod!{query, { 
                getname!(query);
                getsrc!(query);
                getpath!(query);
                get_deps!(query);
                get_crates!(query);
                mkinclude!(query);
                 
            }}
mkmod!{dep_graph, { 
                getname!(dep_graph);
                getsrc!(dep_graph);
                getpath!(dep_graph);
                get_deps!(dep_graph);
                get_crates!(dep_graph);
                mkinclude!(dep_graph);
                 
            }}
mkitem!{extern crate self as rustc_middle ;}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}