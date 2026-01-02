mkmod!{raw, { 
                getname!(raw);
                getsrc!(raw);
                getpath!(raw);
                get_deps!(raw);
                get_crates!(raw);
                mkinclude!(raw);
                 
            }}
mkmod!{owned, { 
                getname!(owned);
                getsrc!(owned);
                getpath!(owned);
                get_deps!(owned);
                get_crates!(owned);
                mkinclude!(owned);
                 
            }}
mkmod!{net, { 
                getname!(net);
                getsrc!(net);
                getpath!(net);
                get_deps!(net);
                get_crates!(net);
                mkinclude!(net);
                 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{# [stable (feature = "os_fd" , since = "1.66.0")] pub use owned :: * ;}
mkuse!{# [stable (feature = "os_fd" , since = "1.66.0")] pub use raw :: * ;}