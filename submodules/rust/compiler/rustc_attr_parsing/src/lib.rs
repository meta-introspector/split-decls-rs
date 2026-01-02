mkmod!{attributes, { 
                getname!(attributes);
                getsrc!(attributes);
                getpath!(attributes);
                get_deps!(attributes);
                get_crates!(attributes);
                mkinclude!(attributes);
                 
            }}
mkmod!{context, { 
                getname!(context);
                getsrc!(context);
                getpath!(context);
                get_deps!(context);
                get_crates!(context);
                mkinclude!(context);
                 
            }}
mkmod!{interface, { 
                getname!(interface);
                getsrc!(interface);
                getpath!(interface);
                get_deps!(interface);
                get_crates!(interface);
                mkinclude!(interface);
                 
            }}
mkmod!{parser, { 
                getname!(parser);
                getsrc!(parser);
                getpath!(parser);
                get_deps!(parser);
                get_crates!(parser);
                mkinclude!(parser);
                 
            }}
mkmod!{lints, { 
                getname!(lints);
                getsrc!(lints);
                getpath!(lints);
                get_deps!(lints);
                get_crates!(lints);
                mkinclude!(lints);
                 
            }}
mkmod!{session_diagnostics, { 
                getname!(session_diagnostics);
                getsrc!(session_diagnostics);
                getpath!(session_diagnostics);
                get_deps!(session_diagnostics);
                get_crates!(session_diagnostics);
                mkinclude!(session_diagnostics);
                 
            }}
mkmod!{target_checking, { 
                getname!(target_checking);
                getsrc!(target_checking);
                getpath!(target_checking);
                get_deps!(target_checking);
                get_crates!(target_checking);
                mkinclude!(target_checking);
                 
            }}
mkmod!{validate_attr, { 
                getname!(validate_attr);
                getsrc!(validate_attr);
                getpath!(validate_attr);
                get_deps!(validate_attr);
                get_crates!(validate_attr);
                mkinclude!(validate_attr);
                 
            }}
mkuse!{pub use attributes :: cfg :: { CFG_TEMPLATE , EvalConfigResult , eval_config_entry , parse_cfg_attr } ;}
mkuse!{pub use attributes :: cfg_old :: * ;}
mkuse!{pub use attributes :: util :: { is_builtin_attr , is_doc_alias_attrs_contain_symbol , parse_version } ;}
mkuse!{pub use context :: { Early , Late , OmitDoc , ShouldEmit } ;}
mkuse!{pub use interface :: AttributeParser ;}
mkuse!{pub use lints :: emit_attribute_lint ;}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}