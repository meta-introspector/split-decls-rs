mkmod!{build, { 
                getname!(build);
                getsrc!(build);
                getpath!(build);
                get_deps!(build);
                get_crates!(build);
                mkinclude!(build);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{mbe, { 
                getname!(mbe);
                getsrc!(mbe);
                getpath!(mbe);
                get_deps!(mbe);
                get_crates!(mbe);
                mkinclude!(mbe);
                 
            }}
mkmod!{placeholders, { 
                getname!(placeholders);
                getsrc!(placeholders);
                getpath!(placeholders);
                get_deps!(placeholders);
                get_crates!(placeholders);
                mkinclude!(placeholders);
                 
            }}
mkmod!{proc_macro_server, { 
                getname!(proc_macro_server);
                getsrc!(proc_macro_server);
                getpath!(proc_macro_server);
                get_deps!(proc_macro_server);
                get_crates!(proc_macro_server);
                mkinclude!(proc_macro_server);
                 
            }}
mkmod!{stats, { 
                getname!(stats);
                getsrc!(stats);
                getpath!(stats);
                get_deps!(stats);
                get_crates!(stats);
                mkinclude!(stats);
                 
            }}
mkuse!{pub use mbe :: macro_rules :: { MacroRulesMacroExpander , compile_declarative_macro } ;}
mkmod!{base, { 
                getname!(base);
                getsrc!(base);
                getpath!(base);
                get_deps!(base);
                get_crates!(base);
                mkinclude!(base);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                 
            }}
mkmod!{expand, { 
                getname!(expand);
                getsrc!(expand);
                getpath!(expand);
                get_deps!(expand);
                get_crates!(expand);
                mkinclude!(expand);
                 
            }}
mkmod!{module, { 
                getname!(module);
                getsrc!(module);
                getpath!(module);
                get_deps!(module);
                get_crates!(module);
                mkinclude!(module);
                 
            }}
mkmod!{proc_macro, { 
                getname!(proc_macro);
                getsrc!(proc_macro);
                getpath!(proc_macro);
                get_deps!(proc_macro);
                get_crates!(proc_macro);
                mkinclude!(proc_macro);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}