mkuse!{# [doc (inline)] # [stable (feature = "alloc_c_string" , since = "1.64.0")] pub use self :: c_str :: CString ;}
mkuse!{# [doc (inline)] # [stable (feature = "alloc_c_string" , since = "1.64.0")] pub use self :: c_str :: { FromVecWithNulError , IntoStringError , NulError } ;}
mkmod!{c_str, { 
                getname!(c_str);
                getsrc!(c_str);
                getpath!(c_str);
                get_deps!(c_str);
                get_crates!(c_str);
                mkinclude!(c_str);
                 
            }}