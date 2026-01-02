mkmod!{futex, { 
                getname!(futex);
                getsrc!(futex);
                getpath!(futex);
                get_deps!(futex);
                get_crates!(futex);
                mkinclude!(futex);
                 
            }}
mkmod!{os, { 
                getname!(os);
                getsrc!(os);
                getpath!(os);
                get_deps!(os);
                get_crates!(os);
                mkinclude!(os);
                 
            }}
mkmod!{pipe, { 
                getname!(pipe);
                getsrc!(pipe);
                getpath!(pipe);
                get_deps!(pipe);
                get_crates!(pipe);
                mkinclude!(pipe);
                 
            }}
mkmod!{time, { 
                getname!(time);
                getsrc!(time);
                getpath!(time);
                get_deps!(time);
                get_crates!(time);
                mkinclude!(time);
                 
            }}
mkmod!{common, { 
                getname!(common);
                getsrc!(common);
                getpath!(common);
                get_deps!(common);
                get_crates!(common);
                mkinclude!(common);
                 
            }}
mkuse!{pub use common :: * ;}
mkmod!{helpers, { 
                getname!(helpers);
                getsrc!(helpers);
                getpath!(helpers);
                get_deps!(helpers);
                get_crates!(helpers);
                mkinclude!(helpers);
                 
            }}
mkuse!{pub (crate) use helpers :: { abort_internal , decode_error_kind , err2io , is_interrupted } ;}
mkmod!{cabi_realloc, { 
                getname!(cabi_realloc);
                getsrc!(cabi_realloc);
                getpath!(cabi_realloc);
                get_deps!(cabi_realloc);
                get_crates!(cabi_realloc);
                mkinclude!(cabi_realloc);
                 
            }}