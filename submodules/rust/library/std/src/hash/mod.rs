mkmod!{random, { 
                getname!(random);
                getsrc!(random);
                getpath!(random);
                get_deps!(random);
                get_crates!(random);
                mkinclude!(random);
                 
            }}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: hash :: * ;}
mkuse!{# [stable (feature = "std_hash_exports" , since = "1.76.0")] pub use self :: random :: { DefaultHasher , RandomState } ;}