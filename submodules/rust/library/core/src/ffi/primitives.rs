mkitem!{macro_rules ! type_alias { { $ Docfile : tt , $ Alias : ident = $ Real : ty ; $ ($ Cfg : tt) * } => { # [doc = include_str ! ($ Docfile)] $ ($ Cfg) * # [stable (feature = "core_ffi_c" , since = "1.64.0")] pub type $ Alias = $ Real ; } }}
mkitem!{type_alias ! { "c_char.md" , c_char = c_char_definition :: c_char ; # [doc (cfg (all ()))] }}
mkitem!{type_alias ! { "c_schar.md" , c_schar = i8 ; }}
mkitem!{type_alias ! { "c_uchar.md" , c_uchar = u8 ; }}
mkitem!{type_alias ! { "c_short.md" , c_short = i16 ; }}
mkitem!{type_alias ! { "c_ushort.md" , c_ushort = u16 ; }}
mkitem!{type_alias ! { "c_int.md" , c_int = c_int_definition :: c_int ; # [doc (cfg (all ()))] }}
mkitem!{type_alias ! { "c_uint.md" , c_uint = c_int_definition :: c_uint ; # [doc (cfg (all ()))] }}
mkitem!{type_alias ! { "c_long.md" , c_long = c_long_definition :: c_long ; # [doc (cfg (all ()))] }}
mkitem!{type_alias ! { "c_ulong.md" , c_ulong = c_long_definition :: c_ulong ; # [doc (cfg (all ()))] }}
mkitem!{type_alias ! { "c_longlong.md" , c_longlong = i64 ; }}
mkitem!{type_alias ! { "c_ulonglong.md" , c_ulonglong = u64 ; }}
mkitem!{type_alias ! { "c_float.md" , c_float = f32 ; }}
mkitem!{type_alias ! { "c_double.md" , c_double = f64 ; }}
mkmod!{c_char_definition, { 
                getname!(c_char_definition);
                getsrc!(c_char_definition);
                getpath!(c_char_definition);
                get_deps!(c_char_definition);
                get_crates!(c_char_definition);
                mkinclude!(c_char_definition);
                mkitem!{crate :: cfg_select ! { all (not (windows) , not (target_vendor = "apple") , not (target_os = "vita") , any (target_arch = "aarch64" , target_arch = "arm" , target_arch = "csky" , target_arch = "hexagon" , target_arch = "msp430" , target_arch = "powerpc" , target_arch = "powerpc64" , target_arch = "riscv32" , target_arch = "riscv64" , target_arch = "s390x" , target_arch = "xtensa" ,)) => { pub (super) type c_char = u8 ; } _ => { pub (super) type c_char = i8 ; } }} 
            }}
mkmod!{c_long_definition, { 
                getname!(c_long_definition);
                getsrc!(c_long_definition);
                getpath!(c_long_definition);
                get_deps!(c_long_definition);
                get_crates!(c_long_definition);
                mkinclude!(c_long_definition);
                mkitem!{crate :: cfg_select ! { any (all (target_pointer_width = "64" , not (windows)) , all (target_arch = "wasm32" , target_os = "linux")) => { pub (super) type c_long = i64 ; pub (super) type c_ulong = u64 ; } _ => { pub (super) type c_long = i32 ; pub (super) type c_ulong = u32 ; } }} 
            }}
mkitem!{# [doc = " Equivalent to C's `size_t` type, from `stddef.h` (or `cstddef` for C++)."] # [doc = ""] # [doc = " This type is currently always [`usize`], however in the future there may be"] # [doc = " platforms where this is not the case."] # [unstable (feature = "c_size_t" , issue = "88345")] pub type c_size_t = usize ;}
mkitem!{# [doc = " Equivalent to C's `ptrdiff_t` type, from `stddef.h` (or `cstddef` for C++)."] # [doc = ""] # [doc = " This type is currently always [`isize`], however in the future there may be"] # [doc = " platforms where this is not the case."] # [unstable (feature = "c_size_t" , issue = "88345")] pub type c_ptrdiff_t = isize ;}
mkitem!{# [doc = " Equivalent to C's `ssize_t` (on POSIX) or `SSIZE_T` (on Windows) type."] # [doc = ""] # [doc = " This type is currently always [`isize`], however in the future there may be"] # [doc = " platforms where this is not the case."] # [unstable (feature = "c_size_t" , issue = "88345")] pub type c_ssize_t = isize ;}
mkmod!{c_int_definition, { 
                getname!(c_int_definition);
                getsrc!(c_int_definition);
                getpath!(c_int_definition);
                get_deps!(c_int_definition);
                get_crates!(c_int_definition);
                mkinclude!(c_int_definition);
                mkitem!{crate :: cfg_select ! { any (target_arch = "avr" , target_arch = "msp430") => { pub (super) type c_int = i16 ; pub (super) type c_uint = u16 ; } _ => { pub (super) type c_int = i32 ; pub (super) type c_uint = u32 ; } }} 
            }}