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
mkmod!{traits, { 
                getname!(traits);
                getsrc!(traits);
                getpath!(traits);
                get_deps!(traits);
                get_crates!(traits);
                mkinclude!(traits);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}