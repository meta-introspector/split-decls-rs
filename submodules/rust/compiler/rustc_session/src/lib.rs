mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{utils, { 
                getname!(utils);
                getsrc!(utils);
                getpath!(utils);
                get_deps!(utils);
                get_crates!(utils);
                mkinclude!(utils);
                 
            }}
mkuse!{pub use lint :: { declare_lint , declare_lint_pass , declare_tool_lint , impl_lint_pass } ;}
mkuse!{pub use rustc_lint_defs as lint ;}
mkmod!{parse, { 
                getname!(parse);
                getsrc!(parse);
                getpath!(parse);
                get_deps!(parse);
                get_crates!(parse);
                mkinclude!(parse);
                 
            }}
mkmod!{code_stats, { 
                getname!(code_stats);
                getsrc!(code_stats);
                getpath!(code_stats);
                get_deps!(code_stats);
                get_crates!(code_stats);
                mkinclude!(code_stats);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                 
            }}
mkmod!{cstore, { 
                getname!(cstore);
                getsrc!(cstore);
                getpath!(cstore);
                get_deps!(cstore);
                get_crates!(cstore);
                mkinclude!(cstore);
                 
            }}
mkmod!{filesearch, { 
                getname!(filesearch);
                getsrc!(filesearch);
                getpath!(filesearch);
                get_deps!(filesearch);
                get_crates!(filesearch);
                mkinclude!(filesearch);
                 
            }}
mkmod!{options, { 
                getname!(options);
                getsrc!(options);
                getpath!(options);
                get_deps!(options);
                get_crates!(options);
                mkinclude!(options);
                 
            }}
mkmod!{search_paths, { 
                getname!(search_paths);
                getsrc!(search_paths);
                getpath!(search_paths);
                get_deps!(search_paths);
                get_crates!(search_paths);
                mkinclude!(search_paths);
                 
            }}
mkmod!{session, { 
                getname!(session);
                getsrc!(session);
                getpath!(session);
                get_deps!(session);
                get_crates!(session);
                mkinclude!(session);
                 
            }}
mkuse!{pub use session :: * ;}
mkmod!{output, { 
                getname!(output);
                getsrc!(output);
                getpath!(output);
                get_deps!(output);
                get_crates!(output);
                mkinclude!(output);
                 
            }}
mkuse!{pub use getopts ;}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}
mkitem!{mktrait!{# [doc = " Requirements for a `StableHashingContext` to be used in this crate."] # [doc = " This is a hack to allow using the `HashStable_Generic` derive macro"] # [doc = " instead of implementing everything in `rustc_middle`."] pub trait HashStableContext : rustc_ast :: HashStableContext + rustc_hir :: HashStableContext { }}}