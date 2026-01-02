mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{vector, { 
                getname!(vector);
                getsrc!(vector);
                getpath!(vector);
                get_deps!(vector);
                get_crates!(vector);
                mkinclude!(vector);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_s390x" , issue = "130869")] pub use self :: vector :: * ;}