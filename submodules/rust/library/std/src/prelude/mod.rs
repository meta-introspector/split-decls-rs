mkmod!{v1, { 
                getname!(v1);
                getsrc!(v1);
                getpath!(v1);
                get_deps!(v1);
                get_crates!(v1);
                mkinclude!(v1);
                 
            }}
mkmod!{rust_2015, { 
                getname!(rust_2015);
                getsrc!(rust_2015);
                getpath!(rust_2015);
                get_deps!(rust_2015);
                get_crates!(rust_2015);
                mkinclude!(rust_2015);
                mkuse!{# [stable (feature = "prelude_2015" , since = "1.55.0")] # [doc (no_inline)] pub use super :: v1 :: * ;} 
            }}
mkmod!{rust_2018, { 
                getname!(rust_2018);
                getsrc!(rust_2018);
                getpath!(rust_2018);
                get_deps!(rust_2018);
                get_crates!(rust_2018);
                mkinclude!(rust_2018);
                mkuse!{# [stable (feature = "prelude_2018" , since = "1.55.0")] # [doc (no_inline)] pub use super :: v1 :: * ;} 
            }}
mkmod!{rust_2021, { 
                getname!(rust_2021);
                getsrc!(rust_2021);
                getpath!(rust_2021);
                get_deps!(rust_2021);
                get_crates!(rust_2021);
                mkinclude!(rust_2021);
                mkuse!{# [stable (feature = "prelude_2021" , since = "1.55.0")] # [doc (no_inline)] pub use super :: v1 :: * ;}
mkuse!{# [stable (feature = "prelude_2021" , since = "1.55.0")] # [doc (no_inline)] pub use core :: prelude :: rust_2021 :: * ;} 
            }}
mkmod!{rust_2024, { 
                getname!(rust_2024);
                getsrc!(rust_2024);
                getpath!(rust_2024);
                get_deps!(rust_2024);
                get_crates!(rust_2024);
                mkinclude!(rust_2024);
                mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use super :: v1 :: * ;}
mkuse!{# [stable (feature = "prelude_2024" , since = "1.85.0")] # [doc (no_inline)] pub use core :: prelude :: rust_2024 :: * ;} 
            }}
mkmod!{rust_future, { 
                getname!(rust_future);
                getsrc!(rust_future);
                getpath!(rust_future);
                get_deps!(rust_future);
                get_crates!(rust_future);
                mkinclude!(rust_future);
                mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (no_inline)] pub use super :: v1 :: * ;}
mkuse!{# [unstable (feature = "prelude_next" , issue = "none")] # [doc (no_inline)] pub use core :: prelude :: rust_future :: * ;} 
            }}