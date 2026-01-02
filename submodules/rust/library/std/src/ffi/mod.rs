mkmod!{c_str, { 
                getname!(c_str);
                getsrc!(c_str);
                getpath!(c_str);
                get_deps!(c_str);
                get_crates!(c_str);
                mkinclude!(c_str);
                 
            }}
mkuse!{# [stable (feature = "core_c_void" , since = "1.30.0")] pub use core :: ffi :: c_void ;}
mkuse!{# [unstable (feature = "c_variadic" , reason = "the `c_variadic` feature has not been properly tested on \
              all supported platforms" , issue = "44930")] pub use core :: ffi :: { VaArgSafe , VaList , VaListImpl } ;}
mkuse!{# [stable (feature = "core_ffi_c" , since = "1.64.0")] pub use core :: ffi :: { c_char , c_double , c_float , c_int , c_long , c_longlong , c_schar , c_short , c_uchar , c_uint , c_ulong , c_ulonglong , c_ushort , } ;}
mkuse!{# [unstable (feature = "c_size_t" , issue = "88345")] pub use core :: ffi :: { c_ptrdiff_t , c_size_t , c_ssize_t } ;}
mkuse!{# [doc (inline)] # [stable (feature = "cstr_from_bytes_until_nul" , since = "1.69.0")] pub use self :: c_str :: FromBytesUntilNulError ;}
mkuse!{# [doc (inline)] # [stable (feature = "cstr_from_bytes" , since = "1.10.0")] pub use self :: c_str :: FromBytesWithNulError ;}
mkuse!{# [doc (inline)] # [stable (feature = "cstring_from_vec_with_nul" , since = "1.58.0")] pub use self :: c_str :: FromVecWithNulError ;}
mkuse!{# [doc (inline)] # [stable (feature = "cstring_into" , since = "1.7.0")] pub use self :: c_str :: IntoStringError ;}
mkuse!{# [doc (inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use self :: c_str :: NulError ;}
mkuse!{# [doc (inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use self :: c_str :: { CStr , CString } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (inline)] pub use self :: os_str :: { OsStr , OsString } ;}
mkmod!{os_str, { 
                getname!(os_str);
                getsrc!(os_str);
                getpath!(os_str);
                get_deps!(os_str);
                get_crates!(os_str);
                mkinclude!(os_str);
                 
            }}