mkmod!{handle, { 
                getname!(handle);
                getsrc!(handle);
                getpath!(handle);
                get_deps!(handle);
                get_crates!(handle);
                mkinclude!(handle);
                 
            }}
mkmod!{raw, { 
                getname!(raw);
                getsrc!(raw);
                getpath!(raw);
                get_deps!(raw);
                get_crates!(raw);
                mkinclude!(raw);
                 
            }}
mkmod!{socket, { 
                getname!(socket);
                getsrc!(socket);
                getpath!(socket);
                get_deps!(socket);
                get_crates!(socket);
                mkinclude!(socket);
                 
            }}
mkuse!{# [stable (feature = "io_safety" , since = "1.63.0")] pub use handle :: * ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use raw :: * ;}
mkuse!{# [stable (feature = "io_safety" , since = "1.63.0")] pub use socket :: * ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}