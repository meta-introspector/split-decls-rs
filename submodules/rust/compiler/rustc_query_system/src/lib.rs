mkmod!{cache, { 
                getname!(cache);
                getsrc!(cache);
                getpath!(cache);
                get_deps!(cache);
                get_crates!(cache);
                mkinclude!(cache);
                 
            }}
mkmod!{dep_graph, { 
                getname!(dep_graph);
                getsrc!(dep_graph);
                getpath!(dep_graph);
                get_deps!(dep_graph);
                get_crates!(dep_graph);
                mkinclude!(dep_graph);
                 
            }}
mkmod!{error, { 
                getname!(error);
                getsrc!(error);
                getpath!(error);
                get_deps!(error);
                get_crates!(error);
                mkinclude!(error);
                 
            }}
mkmod!{ich, { 
                getname!(ich);
                getsrc!(ich);
                getpath!(ich);
                get_deps!(ich);
                get_crates!(ich);
                mkinclude!(ich);
                 
            }}
mkmod!{query, { 
                getname!(query);
                getsrc!(query);
                getpath!(query);
                get_deps!(query);
                get_crates!(query);
                mkinclude!(query);
                 
            }}
mkmod!{values, { 
                getname!(values);
                getsrc!(values);
                getpath!(values);
                get_deps!(values);
                get_crates!(values);
                mkinclude!(values);
                 
            }}
mkuse!{pub use error :: { HandleCycleError , QueryOverflow , QueryOverflowNote } ;}
mkuse!{pub use values :: Value ;}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}