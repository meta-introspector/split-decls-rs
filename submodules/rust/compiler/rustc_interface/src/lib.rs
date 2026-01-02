mkmod!{callbacks, { 
                getname!(callbacks);
                getsrc!(callbacks);
                getpath!(callbacks);
                get_deps!(callbacks);
                get_crates!(callbacks);
                mkinclude!(callbacks);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{interface, { 
                getname!(interface);
                getsrc!(interface);
                getpath!(interface);
                get_deps!(interface);
                get_crates!(interface);
                mkinclude!(interface);
                 
            }}
mkmod!{limits, { 
                getname!(limits);
                getsrc!(limits);
                getpath!(limits);
                get_deps!(limits);
                get_crates!(limits);
                mkinclude!(limits);
                 
            }}
mkmod!{passes, { 
                getname!(passes);
                getsrc!(passes);
                getpath!(passes);
                get_deps!(passes);
                get_crates!(passes);
                mkinclude!(passes);
                 
            }}
mkmod!{proc_macro_decls, { 
                getname!(proc_macro_decls);
                getsrc!(proc_macro_decls);
                getpath!(proc_macro_decls);
                get_deps!(proc_macro_decls);
                get_crates!(proc_macro_decls);
                mkinclude!(proc_macro_decls);
                 
            }}
mkmod!{queries, { 
                getname!(queries);
                getsrc!(queries);
                getpath!(queries);
                get_deps!(queries);
                get_crates!(queries);
                mkinclude!(queries);
                 
            }}
mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                 
            }}
mkuse!{pub use callbacks :: setup_callbacks ;}
mkuse!{pub use interface :: { Config , run_compiler } ;}
mkuse!{pub use passes :: { DEFAULT_QUERY_PROVIDERS , create_and_enter_global_ctxt , parse } ;}
mkuse!{pub use queries :: Linker ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}