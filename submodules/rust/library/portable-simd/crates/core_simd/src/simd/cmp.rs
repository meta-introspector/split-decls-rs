mkmod!{eq, { 
                getname!(eq);
                getsrc!(eq);
                getpath!(eq);
                get_deps!(eq);
                get_crates!(eq);
                mkinclude!(eq);
                 
            }}
mkmod!{ord, { 
                getname!(ord);
                getsrc!(ord);
                getpath!(ord);
                get_deps!(ord);
                get_crates!(ord);
                mkinclude!(ord);
                 
            }}
mkuse!{pub use eq :: * ;}
mkuse!{pub use ord :: * ;}