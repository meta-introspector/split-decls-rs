mkmod!{ast_validation, { 
                getname!(ast_validation);
                getsrc!(ast_validation);
                getpath!(ast_validation);
                get_deps!(ast_validation);
                get_crates!(ast_validation);
                mkinclude!(ast_validation);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{feature_gate, { 
                getname!(feature_gate);
                getsrc!(feature_gate);
                getpath!(feature_gate);
                get_deps!(feature_gate);
                get_crates!(feature_gate);
                mkinclude!(feature_gate);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}