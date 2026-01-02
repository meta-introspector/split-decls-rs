mkmod!{atomic, { 
                getname!(atomic);
                getsrc!(atomic);
                getpath!(atomic);
                get_deps!(atomic);
                get_crates!(atomic);
                mkinclude!(atomic);
                 
            }}
mkmod!{exclusive, { 
                getname!(exclusive);
                getsrc!(exclusive);
                getpath!(exclusive);
                get_deps!(exclusive);
                get_crates!(exclusive);
                mkinclude!(exclusive);
                 
            }}
mkuse!{# [unstable (feature = "exclusive_wrapper" , issue = "98407")] pub use exclusive :: Exclusive ;}