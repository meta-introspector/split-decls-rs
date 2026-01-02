mkmod!{prelude, { 
                getname!(prelude);
                getsrc!(prelude);
                getpath!(prelude);
                get_deps!(prelude);
                get_crates!(prelude);
                mkinclude!(prelude);
                 
            }}
mkuse!{# [prelude_import] # [allow (unused)] use prelude :: rust_2024 :: * ;}
mkitem!{# [cfg (test)] extern crate test ;}
mkitem!{# [allow (unused_imports)] # [macro_use] extern crate alloc as alloc_crate ;}
mkitem!{# [doc (masked)] # [allow (unused_extern_crates)] # [cfg (not (all (windows , target_env = "msvc")))] extern crate libc ;}
mkitem!{# [doc (masked)] # [allow (unused_extern_crates)] extern crate unwind ;}
mkitem!{# [doc (masked)] # [allow (unused_extern_crates)] # [cfg (all (not (all (windows , target_env = "msvc" , not (target_vendor = "uwp"))) , feature = "miniz_oxide"))] extern crate miniz_oxide ;}
mkitem!{# [cfg (test)] extern crate std as realstd ;}
mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{rt, { 
                getname!(rt);
                getsrc!(rt);
                getpath!(rt);
                get_deps!(rt);
                get_crates!(rt);
                mkinclude!(rt);
                 
            }}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: any ;}
mkuse!{# [stable (feature = "core_array" , since = "1.35.0")] pub use core :: array ;}
mkuse!{# [unstable (feature = "async_iterator" , issue = "79024")] pub use core :: async_iter ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: cell ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: char ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: clone ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: cmp ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: convert ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: default ;}
mkuse!{# [stable (feature = "futures_api" , since = "1.36.0")] pub use core :: future ;}
mkuse!{# [stable (feature = "core_hint" , since = "1.27.0")] pub use core :: hint ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: i8 ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: i16 ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: i32 ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: i64 ;}
mkuse!{# [stable (feature = "i128" , since = "1.26.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: i128 ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: intrinsics ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: isize ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: iter ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: marker ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: mem ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: ops ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: option ;}
mkuse!{# [stable (feature = "pin" , since = "1.33.0")] pub use core :: pin ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: ptr ;}
mkuse!{# [unstable (feature = "new_range_api" , issue = "125687")] pub use core :: range ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: result ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: u8 ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: u16 ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: u32 ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: u64 ;}
mkuse!{# [stable (feature = "i128" , since = "1.26.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: u128 ;}
mkuse!{# [unstable (feature = "unsafe_binders" , issue = "130516")] pub use core :: unsafe_binder ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: usize ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: borrow ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: boxed ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: fmt ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: format ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: rc ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: slice ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: str ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: string ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: vec ;}
mkmod!{f128, { 
                getname!(f128);
                getsrc!(f128);
                getpath!(f128);
                get_deps!(f128);
                get_crates!(f128);
                mkinclude!(f128);
                 
            }}
mkmod!{f16, { 
                getname!(f16);
                getsrc!(f16);
                getpath!(f16);
                get_deps!(f16);
                get_crates!(f16);
                mkinclude!(f16);
                 
            }}
mkmod!{f32, { 
                getname!(f32);
                getsrc!(f32);
                getpath!(f32);
                get_deps!(f32);
                get_crates!(f32);
                mkinclude!(f32);
                 
            }}
mkmod!{f64, { 
                getname!(f64);
                getsrc!(f64);
                getpath!(f64);
                get_deps!(f64);
                get_crates!(f64);
                mkinclude!(f64);
                 
            }}
mkmod!{thread, { 
                getname!(thread);
                getsrc!(thread);
                getpath!(thread);
                get_deps!(thread);
                get_crates!(thread);
                mkinclude!(thread);
                 
            }}
mkmod!{ascii, { 
                getname!(ascii);
                getsrc!(ascii);
                getpath!(ascii);
                get_deps!(ascii);
                get_crates!(ascii);
                mkinclude!(ascii);
                 
            }}
mkmod!{backtrace, { 
                getname!(backtrace);
                getsrc!(backtrace);
                getpath!(backtrace);
                get_deps!(backtrace);
                get_crates!(backtrace);
                mkinclude!(backtrace);
                 
            }}
mkmod!{bstr, { 
                getname!(bstr);
                getsrc!(bstr);
                getpath!(bstr);
                get_deps!(bstr);
                get_crates!(bstr);
                mkinclude!(bstr);
                 
            }}
mkmod!{collections, { 
                getname!(collections);
                getsrc!(collections);
                getpath!(collections);
                get_deps!(collections);
                get_crates!(collections);
                mkinclude!(collections);
                 
            }}
mkmod!{env, { 
                getname!(env);
                getsrc!(env);
                getpath!(env);
                get_deps!(env);
                get_crates!(env);
                mkinclude!(env);
                 
            }}
mkmod!{error, { 
                getname!(error);
                getsrc!(error);
                getpath!(error);
                get_deps!(error);
                get_crates!(error);
                mkinclude!(error);
                 
            }}
mkmod!{ffi, { 
                getname!(ffi);
                getsrc!(ffi);
                getpath!(ffi);
                get_deps!(ffi);
                get_crates!(ffi);
                mkinclude!(ffi);
                 
            }}
mkmod!{fs, { 
                getname!(fs);
                getsrc!(fs);
                getpath!(fs);
                get_deps!(fs);
                get_crates!(fs);
                mkinclude!(fs);
                 
            }}
mkmod!{hash, { 
                getname!(hash);
                getsrc!(hash);
                getpath!(hash);
                get_deps!(hash);
                get_crates!(hash);
                mkinclude!(hash);
                 
            }}
mkmod!{io, { 
                getname!(io);
                getsrc!(io);
                getpath!(io);
                get_deps!(io);
                get_crates!(io);
                mkinclude!(io);
                 
            }}
mkmod!{net, { 
                getname!(net);
                getsrc!(net);
                getpath!(net);
                get_deps!(net);
                get_crates!(net);
                mkinclude!(net);
                 
            }}
mkmod!{num, { 
                getname!(num);
                getsrc!(num);
                getpath!(num);
                get_deps!(num);
                get_crates!(num);
                mkinclude!(num);
                 
            }}
mkmod!{os, { 
                getname!(os);
                getsrc!(os);
                getpath!(os);
                get_deps!(os);
                get_crates!(os);
                mkinclude!(os);
                 
            }}
mkmod!{panic, { 
                getname!(panic);
                getsrc!(panic);
                getpath!(panic);
                get_deps!(panic);
                get_crates!(panic);
                mkinclude!(panic);
                 
            }}
mkmod!{pat, { 
                getname!(pat);
                getsrc!(pat);
                getpath!(pat);
                get_deps!(pat);
                get_crates!(pat);
                mkinclude!(pat);
                 
            }}
mkmod!{path, { 
                getname!(path);
                getsrc!(path);
                getpath!(path);
                get_deps!(path);
                get_crates!(path);
                mkinclude!(path);
                 
            }}
mkmod!{process, { 
                getname!(process);
                getsrc!(process);
                getpath!(process);
                get_deps!(process);
                get_crates!(process);
                mkinclude!(process);
                 
            }}
mkmod!{random, { 
                getname!(random);
                getsrc!(random);
                getpath!(random);
                get_deps!(random);
                get_crates!(random);
                mkinclude!(random);
                 
            }}
mkmod!{sync, { 
                getname!(sync);
                getsrc!(sync);
                getpath!(sync);
                get_deps!(sync);
                get_crates!(sync);
                mkinclude!(sync);
                 
            }}
mkmod!{time, { 
                getname!(time);
                getsrc!(time);
                getpath!(time);
                get_deps!(time);
                get_crates!(time);
                mkinclude!(time);
                 
            }}
mkmod!{std_float, { 
                getname!(std_float);
                getsrc!(std_float);
                getpath!(std_float);
                get_deps!(std_float);
                get_crates!(std_float);
                mkinclude!(std_float);
                 
            }}
mkmod!{simd, { 
                getname!(simd);
                getsrc!(simd);
                getpath!(simd);
                get_deps!(simd);
                get_crates!(simd);
                mkinclude!(simd);
                mkuse!{# [doc (inline)] pub use core :: simd :: * ;}
mkuse!{# [doc (inline)] pub use crate :: std_float :: StdFloat ;} 
            }}
mkmod!{autodiff, { 
                getname!(autodiff);
                getsrc!(autodiff);
                getpath!(autodiff);
                get_deps!(autodiff);
                get_crates!(autodiff);
                mkinclude!(autodiff);
                mkuse!{# [doc = " This macro handles automatic differentiation."] pub use core :: autodiff :: { autodiff_forward , autodiff_reverse } ;} 
            }}
mkmod!{task, { 
                getname!(task);
                getsrc!(task);
                getpath!(task);
                get_deps!(task);
                get_crates!(task);
                mkinclude!(task);
                mkuse!{# [doc (inline)] # [stable (feature = "wake_trait" , since = "1.51.0")] pub use alloc :: task :: * ;}
mkuse!{# [doc (inline)] # [stable (feature = "futures_api" , since = "1.36.0")] pub use core :: task :: * ;} 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{# [stable (feature = "simd_arch" , since = "1.27.0")] # [doc (no_inline)] pub use core :: arch :: * ;}
mkuse!{# [stable (feature = "simd_aarch64" , since = "1.60.0")] pub use std_detect :: is_aarch64_feature_detected ;}
mkuse!{# [unstable (feature = "stdarch_arm_feature_detection" , issue = "111190")] pub use std_detect :: is_arm_feature_detected ;}
mkuse!{# [unstable (feature = "is_loongarch_feature_detected" , issue = "117425")] pub use std_detect :: is_loongarch_feature_detected ;}
mkuse!{# [unstable (feature = "is_riscv_feature_detected" , issue = "111192")] pub use std_detect :: is_riscv_feature_detected ;}
mkuse!{# [unstable (feature = "stdarch_s390x_feature_detection" , issue = "135413")] pub use std_detect :: is_s390x_feature_detected ;}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use std_detect :: is_x86_feature_detected ;}
mkuse!{# [unstable (feature = "stdarch_mips_feature_detection" , issue = "111188")] pub use std_detect :: { is_mips_feature_detected , is_mips64_feature_detected } ;}
mkuse!{# [unstable (feature = "stdarch_powerpc_feature_detection" , issue = "111191")] pub use std_detect :: { is_powerpc_feature_detected , is_powerpc64_feature_detected } ;} 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use std_detect :: is_x86_feature_detected ;}
mkmod!{sys, { 
                getname!(sys);
                getsrc!(sys);
                getpath!(sys);
                get_deps!(sys);
                get_crates!(sys);
                mkinclude!(sys);
                 
            }}
mkmod!{sys_common, { 
                getname!(sys_common);
                getsrc!(sys_common);
                getpath!(sys_common);
                get_deps!(sys_common);
                get_crates!(sys_common);
                mkinclude!(sys_common);
                 
            }}
mkmod!{alloc, { 
                getname!(alloc);
                getsrc!(alloc);
                getpath!(alloc);
                get_deps!(alloc);
                get_crates!(alloc);
                mkinclude!(alloc);
                 
            }}
mkmod!{panicking, { 
                getname!(panicking);
                getsrc!(panicking);
                getpath!(panicking);
                get_deps!(panicking);
                get_crates!(panicking);
                mkinclude!(panicking);
                 
            }}
mkmod!{backtrace_rs, { 
                getname!(backtrace_rs);
                getsrc!(backtrace_rs);
                getpath!(backtrace_rs);
                get_deps!(backtrace_rs);
                get_crates!(backtrace_rs);
                mkinclude!(backtrace_rs);
                 
            }}
mkuse!{# [unstable (feature = "cfg_select" , issue = "115585")] pub use core :: cfg_select ;}
mkuse!{# [unstable (feature = "concat_bytes" , issue = "87555" , reason = "`concat_bytes` is not stable enough for use and is subject to change")] pub use core :: concat_bytes ;}
mkuse!{# [stable (feature = "matches_macro" , since = "1.42.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: matches ;}
mkuse!{# [stable (feature = "core_primitive" , since = "1.43.0")] pub use core :: primitive ;}
mkuse!{# [stable (feature = "todo_macro" , since = "1.40.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: todo ;}
mkuse!{# [stable (feature = "builtin_macro_prelude" , since = "1.38.0")] pub use core :: { assert , assert_matches , cfg , column , compile_error , concat , const_format_args , env , file , format_args , format_args_nl , include , include_bytes , include_str , line , log_syntax , module_path , option_env , stringify , trace_macros , } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (deprecated , deprecated_in_future)] pub use core :: { assert_eq , assert_ne , debug_assert , debug_assert_eq , debug_assert_ne , r#try , unimplemented , unreachable , write , writeln , } ;}
mkmod!{from, { 
                getname!(from);
                getsrc!(from);
                getpath!(from);
                get_deps!(from);
                get_crates!(from);
                mkinclude!(from);
                mkuse!{# [unstable (feature = "derive_from" , issue = "144889")] pub use core :: from :: From ;} 
            }}
mkitem!{include ! ("../../core/src/primitive_docs.rs") ;}
mkitem!{include ! ("keyword_docs.rs") ;}
mkmod!{__restricted_std_workaround, { 
                getname!(__restricted_std_workaround);
                getsrc!(__restricted_std_workaround);
                getpath!(__restricted_std_workaround);
                get_deps!(__restricted_std_workaround);
                get_crates!(__restricted_std_workaround);
                mkinclude!(__restricted_std_workaround);
                 
            }}
mkmod!{sealed, { 
                getname!(sealed);
                getsrc!(sealed);
                getpath!(sealed);
                get_deps!(sealed);
                get_crates!(sealed);
                mkinclude!(sealed);
                mkitem!{mktrait!{# [doc = " This trait being unreachable from outside the crate"] # [doc = " prevents outside implementations of our extension traits."] # [doc = " This allows adding more trait methods in the future."] # [unstable (feature = "sealed" , issue = "none")] pub trait Sealed { }}} 
            }}
mkmod!{test_helpers, { 
                getname!(test_helpers);
                getsrc!(test_helpers);
                getpath!(test_helpers);
                get_deps!(test_helpers);
                get_crates!(test_helpers);
                mkinclude!(test_helpers);
                 
            }}