mkmod!{const_ptr, { 
                getname!(const_ptr);
                getsrc!(const_ptr);
                getpath!(const_ptr);
                get_deps!(const_ptr);
                get_crates!(const_ptr);
                mkinclude!(const_ptr);
                 
            }}
mkmod!{mut_ptr, { 
                getname!(mut_ptr);
                getsrc!(mut_ptr);
                getpath!(mut_ptr);
                get_deps!(mut_ptr);
                get_crates!(mut_ptr);
                mkinclude!(mut_ptr);
                 
            }}
mkmod!{sealed, { 
                getname!(sealed);
                getsrc!(sealed);
                getpath!(sealed);
                get_deps!(sealed);
                get_crates!(sealed);
                mkinclude!(sealed);
                mkitem!{mktrait!{pub trait Sealed { }}} 
            }}
mkuse!{pub use const_ptr :: * ;}
mkuse!{pub use mut_ptr :: * ;}