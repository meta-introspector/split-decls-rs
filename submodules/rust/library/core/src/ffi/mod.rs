mkuse!{# [doc (inline)] # [stable (feature = "core_c_str" , since = "1.64.0")] pub use self :: c_str :: CStr ;}
mkuse!{# [doc (inline)] # [stable (feature = "cstr_from_bytes_until_nul" , since = "1.69.0")] pub use self :: c_str :: FromBytesUntilNulError ;}
mkuse!{# [doc (inline)] # [stable (feature = "core_c_str" , since = "1.64.0")] pub use self :: c_str :: FromBytesWithNulError ;}
mkuse!{use crate :: fmt ;}
mkmod!{c_str, { 
                getname!(c_str);
                getsrc!(c_str);
                getpath!(c_str);
                get_deps!(c_str);
                get_crates!(c_str);
                mkinclude!(c_str);
                 
            }}
mkuse!{# [unstable (feature = "c_variadic" , issue = "44930" , reason = "the `c_variadic` feature has not been properly tested on all supported platforms")] pub use self :: va_list :: { VaArgSafe , VaList , VaListImpl } ;}
mkmod!{va_list, { 
                getname!(va_list);
                getsrc!(va_list);
                getpath!(va_list);
                get_deps!(va_list);
                get_crates!(va_list);
                mkinclude!(va_list);
                 
            }}
mkmod!{primitives, { 
                getname!(primitives);
                getsrc!(primitives);
                getpath!(primitives);
                get_deps!(primitives);
                get_crates!(primitives);
                mkinclude!(primitives);
                 
            }}
mkuse!{# [stable (feature = "core_ffi_c" , since = "1.64.0")] pub use self :: primitives :: { c_char , c_double , c_float , c_int , c_long , c_longlong , c_schar , c_short , c_uchar , c_uint , c_ulong , c_ulonglong , c_ushort , } ;}
mkuse!{# [unstable (feature = "c_size_t" , issue = "88345")] pub use self :: primitives :: { c_ptrdiff_t , c_size_t , c_ssize_t } ;}
mkitem!{mkenum!{# [doc = include_str ! ("c_void.md")] # [lang = "c_void"] # [cfg_attr (not (doc) , repr (u8))] # [stable (feature = "core_c_void" , since = "1.30.0")] pub enum c_void { # [unstable (feature = "c_void_variant" , reason = "temporary implementation detail" , issue = "none")] # [doc (hidden)] __variant1 , # [unstable (feature = "c_void_variant" , reason = "temporary implementation detail" , issue = "none")] # [doc (hidden)] __variant2 , }}}
mkitem!{mkimpl!{# [stable (feature = "std_debug" , since = "1.16.0")] impl fmt :: Debug for c_void { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("c_void") . finish () } }}}
mkitem!{# [cfg (all (windows , target_env = "msvc"))] # [link (name = "/defaultlib:msvcrt" , modifiers = "+verbatim" , cfg (not (target_feature = "crt-static")))] # [link (name = "/defaultlib:libcmt" , modifiers = "+verbatim" , cfg (target_feature = "crt-static"))] unsafe extern "C" { }}