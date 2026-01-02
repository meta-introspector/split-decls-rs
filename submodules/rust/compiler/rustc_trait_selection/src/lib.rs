mkmod!{error_reporting, { 
                getname!(error_reporting);
                getsrc!(error_reporting);
                getpath!(error_reporting);
                get_deps!(error_reporting);
                get_crates!(error_reporting);
                mkinclude!(error_reporting);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{infer, { 
                getname!(infer);
                getsrc!(infer);
                getpath!(infer);
                get_deps!(infer);
                get_crates!(infer);
                mkinclude!(infer);
                 
            }}
mkmod!{opaque_types, { 
                getname!(opaque_types);
                getsrc!(opaque_types);
                getpath!(opaque_types);
                get_deps!(opaque_types);
                get_crates!(opaque_types);
                mkinclude!(opaque_types);
                 
            }}
mkmod!{regions, { 
                getname!(regions);
                getsrc!(regions);
                getpath!(regions);
                get_deps!(regions);
                get_crates!(regions);
                mkinclude!(regions);
                 
            }}
mkmod!{solve, { 
                getname!(solve);
                getsrc!(solve);
                getpath!(solve);
                get_deps!(solve);
                get_crates!(solve);
                mkinclude!(solve);
                 
            }}
mkmod!{traits, { 
                getname!(traits);
                getsrc!(traits);
                getpath!(traits);
                get_deps!(traits);
                get_crates!(traits);
                mkinclude!(traits);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}