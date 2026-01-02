mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                mkmod!{case, { 
                getname!(case);
                getsrc!(case);
                getpath!(case);
                get_deps!(case);
                get_crates!(case);
                mkinclude!(case);
                 
            }}
mkmod!{classify, { 
                getname!(classify);
                getsrc!(classify);
                getpath!(classify);
                get_deps!(classify);
                get_crates!(classify);
                mkinclude!(classify);
                 
            }}
mkmod!{comments, { 
                getname!(comments);
                getsrc!(comments);
                getpath!(comments);
                get_deps!(comments);
                get_crates!(comments);
                mkinclude!(comments);
                 
            }}
mkmod!{literal, { 
                getname!(literal);
                getsrc!(literal);
                getpath!(literal);
                get_deps!(literal);
                get_crates!(literal);
                mkinclude!(literal);
                 
            }}
mkmod!{parser, { 
                getname!(parser);
                getsrc!(parser);
                getpath!(parser);
                get_deps!(parser);
                get_crates!(parser);
                mkinclude!(parser);
                 
            }}
mkmod!{unicode, { 
                getname!(unicode);
                getsrc!(unicode);
                getpath!(unicode);
                get_deps!(unicode);
                get_crates!(unicode);
                mkinclude!(unicode);
                 
            }} 
            }}
mkmod!{ast, { 
                getname!(ast);
                getsrc!(ast);
                getpath!(ast);
                get_deps!(ast);
                get_crates!(ast);
                mkinclude!(ast);
                 
            }}
mkmod!{ast_traits, { 
                getname!(ast_traits);
                getsrc!(ast_traits);
                getpath!(ast_traits);
                get_deps!(ast_traits);
                get_crates!(ast_traits);
                mkinclude!(ast_traits);
                 
            }}
mkmod!{attr, { 
                getname!(attr);
                getsrc!(attr);
                getpath!(attr);
                get_deps!(attr);
                get_crates!(attr);
                mkinclude!(attr);
                 
            }}
mkmod!{entry, { 
                getname!(entry);
                getsrc!(entry);
                getpath!(entry);
                get_deps!(entry);
                get_crates!(entry);
                mkinclude!(entry);
                 
            }}
mkmod!{expand, { 
                getname!(expand);
                getsrc!(expand);
                getpath!(expand);
                get_deps!(expand);
                get_crates!(expand);
                mkinclude!(expand);
                 
            }}
mkmod!{format, { 
                getname!(format);
                getsrc!(format);
                getpath!(format);
                get_deps!(format);
                get_crates!(format);
                mkinclude!(format);
                 
            }}
mkmod!{mut_visit, { 
                getname!(mut_visit);
                getsrc!(mut_visit);
                getpath!(mut_visit);
                get_deps!(mut_visit);
                get_crates!(mut_visit);
                mkinclude!(mut_visit);
                 
            }}
mkmod!{node_id, { 
                getname!(node_id);
                getsrc!(node_id);
                getpath!(node_id);
                get_deps!(node_id);
                get_crates!(node_id);
                mkinclude!(node_id);
                 
            }}
mkmod!{token, { 
                getname!(token);
                getsrc!(token);
                getpath!(token);
                get_deps!(token);
                get_crates!(token);
                mkinclude!(token);
                 
            }}
mkmod!{tokenstream, { 
                getname!(tokenstream);
                getsrc!(tokenstream);
                getpath!(tokenstream);
                get_deps!(tokenstream);
                get_crates!(tokenstream);
                mkinclude!(tokenstream);
                 
            }}
mkmod!{visit, { 
                getname!(visit);
                getsrc!(visit);
                getpath!(visit);
                get_deps!(visit);
                get_crates!(visit);
                mkinclude!(visit);
                 
            }}
mkuse!{pub use self :: ast :: * ;}
mkuse!{pub use self :: ast_traits :: { AstNodeWrapper , HasAttrs , HasNodeId , HasTokens } ;}
mkitem!{mktrait!{# [doc = " Requirements for a `StableHashingContext` to be used in this crate."] # [doc = " This is a hack to allow using the `HashStable_Generic` derive macro"] # [doc = " instead of implementing everything in `rustc_middle`."] pub trait HashStableContext : rustc_span :: HashStableContext { }}}