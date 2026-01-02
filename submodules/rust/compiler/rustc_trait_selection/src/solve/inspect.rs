mkuse!{pub use rustc_next_trait_solver :: solve :: inspect :: * ;}
mkmod!{analyse, { 
                getname!(analyse);
                getsrc!(analyse);
                getpath!(analyse);
                get_deps!(analyse);
                get_crates!(analyse);
                mkinclude!(analyse);
                 
            }}
mkuse!{pub use analyse :: * ;}