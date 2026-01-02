mkmod!{add, { 
                getname!(add);
                getsrc!(add);
                getpath!(add);
                get_deps!(add);
                get_crates!(add);
                mkinclude!(add);
                 
            }}
mkmod!{cmp, { 
                getname!(cmp);
                getsrc!(cmp);
                getpath!(cmp);
                get_deps!(cmp);
                get_crates!(cmp);
                mkinclude!(cmp);
                 
            }}
mkmod!{conv, { 
                getname!(conv);
                getsrc!(conv);
                getpath!(conv);
                get_deps!(conv);
                get_crates!(conv);
                mkinclude!(conv);
                 
            }}
mkmod!{div, { 
                getname!(div);
                getsrc!(div);
                getpath!(div);
                get_deps!(div);
                get_crates!(div);
                mkinclude!(div);
                 
            }}
mkmod!{extend, { 
                getname!(extend);
                getsrc!(extend);
                getpath!(extend);
                get_deps!(extend);
                get_crates!(extend);
                mkinclude!(extend);
                 
            }}
mkmod!{mul, { 
                getname!(mul);
                getsrc!(mul);
                getpath!(mul);
                get_deps!(mul);
                get_crates!(mul);
                mkinclude!(mul);
                 
            }}
mkmod!{pow, { 
                getname!(pow);
                getsrc!(pow);
                getpath!(pow);
                get_deps!(pow);
                get_crates!(pow);
                mkinclude!(pow);
                 
            }}
mkmod!{sub, { 
                getname!(sub);
                getsrc!(sub);
                getpath!(sub);
                get_deps!(sub);
                get_crates!(sub);
                mkinclude!(sub);
                 
            }}
mkmod!{traits, { 
                getname!(traits);
                getsrc!(traits);
                getpath!(traits);
                get_deps!(traits);
                get_crates!(traits);
                mkinclude!(traits);
                 
            }}
mkmod!{trunc, { 
                getname!(trunc);
                getsrc!(trunc);
                getpath!(trunc);
                get_deps!(trunc);
                get_crates!(trunc);
                mkinclude!(trunc);
                 
            }}
mkuse!{# [cfg (not (feature = "unstable-public-internals"))] pub (crate) use traits :: { Float , HalfRep } ;}
mkuse!{# [cfg (feature = "unstable-public-internals")] pub use traits :: { Float , HalfRep } ;}