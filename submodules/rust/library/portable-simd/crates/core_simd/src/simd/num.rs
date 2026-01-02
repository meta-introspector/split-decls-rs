mkmod!{float, { 
                getname!(float);
                getsrc!(float);
                getpath!(float);
                get_deps!(float);
                get_crates!(float);
                mkinclude!(float);
                 
            }}
mkmod!{int, { 
                getname!(int);
                getsrc!(int);
                getpath!(int);
                get_deps!(int);
                get_crates!(int);
                mkinclude!(int);
                 
            }}
mkmod!{uint, { 
                getname!(uint);
                getsrc!(uint);
                getpath!(uint);
                get_deps!(uint);
                get_crates!(uint);
                mkinclude!(uint);
                 
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
mkuse!{pub use float :: * ;}
mkuse!{pub use int :: * ;}
mkuse!{pub use uint :: * ;}