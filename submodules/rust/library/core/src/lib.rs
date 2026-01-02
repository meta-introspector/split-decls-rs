mkitem!{# [allow (unused_extern_crates)] extern crate self as core ;}
mkmod!{prelude, { 
                getname!(prelude);
                getsrc!(prelude);
                getpath!(prelude);
                get_deps!(prelude);
                get_crates!(prelude);
                mkinclude!(prelude);
                 
            }}
mkuse!{# [prelude_import] # [allow (unused)] use prelude :: rust_2024 :: * ;}
mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{assert_matches, { 
                getname!(assert_matches);
                getsrc!(assert_matches);
                getpath!(assert_matches);
                get_deps!(assert_matches);
                get_crates!(assert_matches);
                mkinclude!(assert_matches);
                mkuse!{# [unstable (feature = "assert_matches" , issue = "82775")] pub use crate :: macros :: { assert_matches , debug_assert_matches } ;} 
            }}
mkmod!{from, { 
                getname!(from);
                getsrc!(from);
                getpath!(from);
                get_deps!(from);
                get_crates!(from);
                mkinclude!(from);
                mkuse!{# [unstable (feature = "derive_from" , issue = "144889")] pub use crate :: macros :: builtin :: From ;} 
            }}
mkmod!{autodiff, { 
                getname!(autodiff);
                getsrc!(autodiff);
                getpath!(autodiff);
                get_deps!(autodiff);
                get_crates!(autodiff);
                mkinclude!(autodiff);
                mkuse!{# [unstable (feature = "autodiff" , issue = "124509")] pub use crate :: macros :: builtin :: { autodiff_forward , autodiff_reverse } ;} 
            }}
mkmod!{contracts, { 
                getname!(contracts);
                getsrc!(contracts);
                getpath!(contracts);
                get_deps!(contracts);
                get_crates!(contracts);
                mkinclude!(contracts);
                 
            }}
mkuse!{# [unstable (feature = "cfg_select" , issue = "115585")] pub use crate :: macros :: cfg_select ;}
mkmod!{internal_macros, { 
                getname!(internal_macros);
                getsrc!(internal_macros);
                getpath!(internal_macros);
                get_deps!(internal_macros);
                get_crates!(internal_macros);
                mkinclude!(internal_macros);
                 
            }}
mkmod!{legacy_int_modules, { 
                getname!(legacy_int_modules);
                getsrc!(legacy_int_modules);
                getpath!(legacy_int_modules);
                get_deps!(legacy_int_modules);
                get_crates!(legacy_int_modules);
                mkinclude!(legacy_int_modules);
                 
            }}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [allow (clippy :: useless_attribute)] # [allow (deprecated_in_future)] pub use legacy_int_modules :: { i8 , i16 , i32 , i64 , isize , u8 , u16 , u32 , u64 , usize } ;}
mkuse!{# [stable (feature = "i128" , since = "1.26.0")] # [allow (clippy :: useless_attribute)] # [allow (deprecated_in_future)] pub use legacy_int_modules :: { i128 , u128 } ;}
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
mkmod!{num, { 
                getname!(num);
                getsrc!(num);
                getpath!(num);
                get_deps!(num);
                get_crates!(num);
                mkinclude!(num);
                 
            }}
mkmod!{hint, { 
                getname!(hint);
                getsrc!(hint);
                getpath!(hint);
                get_deps!(hint);
                get_crates!(hint);
                mkinclude!(hint);
                 
            }}
mkmod!{intrinsics, { 
                getname!(intrinsics);
                getsrc!(intrinsics);
                getpath!(intrinsics);
                get_deps!(intrinsics);
                get_crates!(intrinsics);
                mkinclude!(intrinsics);
                 
            }}
mkmod!{mem, { 
                getname!(mem);
                getsrc!(mem);
                getpath!(mem);
                get_deps!(mem);
                get_crates!(mem);
                mkinclude!(mem);
                 
            }}
mkmod!{ptr, { 
                getname!(ptr);
                getsrc!(ptr);
                getpath!(ptr);
                get_deps!(ptr);
                get_crates!(ptr);
                mkinclude!(ptr);
                 
            }}
mkmod!{ub_checks, { 
                getname!(ub_checks);
                getsrc!(ub_checks);
                getpath!(ub_checks);
                get_deps!(ub_checks);
                get_crates!(ub_checks);
                mkinclude!(ub_checks);
                 
            }}
mkmod!{borrow, { 
                getname!(borrow);
                getsrc!(borrow);
                getpath!(borrow);
                get_deps!(borrow);
                get_crates!(borrow);
                mkinclude!(borrow);
                 
            }}
mkmod!{clone, { 
                getname!(clone);
                getsrc!(clone);
                getpath!(clone);
                get_deps!(clone);
                get_crates!(clone);
                mkinclude!(clone);
                 
            }}
mkmod!{cmp, { 
                getname!(cmp);
                getsrc!(cmp);
                getpath!(cmp);
                get_deps!(cmp);
                get_crates!(cmp);
                mkinclude!(cmp);
                 
            }}
mkmod!{convert, { 
                getname!(convert);
                getsrc!(convert);
                getpath!(convert);
                get_deps!(convert);
                get_crates!(convert);
                mkinclude!(convert);
                 
            }}
mkmod!{default, { 
                getname!(default);
                getsrc!(default);
                getpath!(default);
                get_deps!(default);
                get_crates!(default);
                mkinclude!(default);
                 
            }}
mkmod!{error, { 
                getname!(error);
                getsrc!(error);
                getpath!(error);
                get_deps!(error);
                get_crates!(error);
                mkinclude!(error);
                 
            }}
mkmod!{marker, { 
                getname!(marker);
                getsrc!(marker);
                getpath!(marker);
                get_deps!(marker);
                get_crates!(marker);
                mkinclude!(marker);
                 
            }}
mkmod!{ops, { 
                getname!(ops);
                getsrc!(ops);
                getpath!(ops);
                get_deps!(ops);
                get_crates!(ops);
                mkinclude!(ops);
                 
            }}
mkmod!{any, { 
                getname!(any);
                getsrc!(any);
                getpath!(any);
                get_deps!(any);
                get_crates!(any);
                mkinclude!(any);
                 
            }}
mkmod!{array, { 
                getname!(array);
                getsrc!(array);
                getpath!(array);
                get_deps!(array);
                get_crates!(array);
                mkinclude!(array);
                 
            }}
mkmod!{ascii, { 
                getname!(ascii);
                getsrc!(ascii);
                getpath!(ascii);
                get_deps!(ascii);
                get_crates!(ascii);
                mkinclude!(ascii);
                 
            }}
mkmod!{asserting, { 
                getname!(asserting);
                getsrc!(asserting);
                getpath!(asserting);
                get_deps!(asserting);
                get_crates!(asserting);
                mkinclude!(asserting);
                 
            }}
mkmod!{async_iter, { 
                getname!(async_iter);
                getsrc!(async_iter);
                getpath!(async_iter);
                get_deps!(async_iter);
                get_crates!(async_iter);
                mkinclude!(async_iter);
                 
            }}
mkmod!{bstr, { 
                getname!(bstr);
                getsrc!(bstr);
                getpath!(bstr);
                get_deps!(bstr);
                get_crates!(bstr);
                mkinclude!(bstr);
                 
            }}
mkmod!{cell, { 
                getname!(cell);
                getsrc!(cell);
                getpath!(cell);
                get_deps!(cell);
                get_crates!(cell);
                mkinclude!(cell);
                 
            }}
mkmod!{char, { 
                getname!(char);
                getsrc!(char);
                getpath!(char);
                get_deps!(char);
                get_crates!(char);
                mkinclude!(char);
                 
            }}
mkmod!{ffi, { 
                getname!(ffi);
                getsrc!(ffi);
                getpath!(ffi);
                get_deps!(ffi);
                get_crates!(ffi);
                mkinclude!(ffi);
                 
            }}
mkmod!{io, { 
                getname!(io);
                getsrc!(io);
                getpath!(io);
                get_deps!(io);
                get_crates!(io);
                mkinclude!(io);
                 
            }}
mkmod!{iter, { 
                getname!(iter);
                getsrc!(iter);
                getpath!(iter);
                get_deps!(iter);
                get_crates!(iter);
                mkinclude!(iter);
                 
            }}
mkmod!{net, { 
                getname!(net);
                getsrc!(net);
                getpath!(net);
                get_deps!(net);
                get_crates!(net);
                mkinclude!(net);
                 
            }}
mkmod!{option, { 
                getname!(option);
                getsrc!(option);
                getpath!(option);
                get_deps!(option);
                get_crates!(option);
                mkinclude!(option);
                 
            }}
mkmod!{panic, { 
                getname!(panic);
                getsrc!(panic);
                getpath!(panic);
                get_deps!(panic);
                get_crates!(panic);
                mkinclude!(panic);
                 
            }}
mkmod!{panicking, { 
                getname!(panicking);
                getsrc!(panicking);
                getpath!(panicking);
                get_deps!(panicking);
                get_crates!(panicking);
                mkinclude!(panicking);
                 
            }}
mkmod!{pat, { 
                getname!(pat);
                getsrc!(pat);
                getpath!(pat);
                get_deps!(pat);
                get_crates!(pat);
                mkinclude!(pat);
                 
            }}
mkmod!{pin, { 
                getname!(pin);
                getsrc!(pin);
                getpath!(pin);
                get_deps!(pin);
                get_crates!(pin);
                mkinclude!(pin);
                 
            }}
mkmod!{random, { 
                getname!(random);
                getsrc!(random);
                getpath!(random);
                get_deps!(random);
                get_crates!(random);
                mkinclude!(random);
                 
            }}
mkmod!{range, { 
                getname!(range);
                getsrc!(range);
                getpath!(range);
                get_deps!(range);
                get_crates!(range);
                mkinclude!(range);
                 
            }}
mkmod!{result, { 
                getname!(result);
                getsrc!(result);
                getpath!(result);
                get_deps!(result);
                get_crates!(result);
                mkinclude!(result);
                 
            }}
mkmod!{sync, { 
                getname!(sync);
                getsrc!(sync);
                getpath!(sync);
                get_deps!(sync);
                get_crates!(sync);
                mkinclude!(sync);
                 
            }}
mkmod!{unsafe_binder, { 
                getname!(unsafe_binder);
                getsrc!(unsafe_binder);
                getpath!(unsafe_binder);
                get_deps!(unsafe_binder);
                get_crates!(unsafe_binder);
                mkinclude!(unsafe_binder);
                 
            }}
mkmod!{fmt, { 
                getname!(fmt);
                getsrc!(fmt);
                getpath!(fmt);
                get_deps!(fmt);
                get_crates!(fmt);
                mkinclude!(fmt);
                 
            }}
mkmod!{hash, { 
                getname!(hash);
                getsrc!(hash);
                getpath!(hash);
                get_deps!(hash);
                get_crates!(hash);
                mkinclude!(hash);
                 
            }}
mkmod!{slice, { 
                getname!(slice);
                getsrc!(slice);
                getpath!(slice);
                get_deps!(slice);
                get_crates!(slice);
                mkinclude!(slice);
                 
            }}
mkmod!{str, { 
                getname!(str);
                getsrc!(str);
                getpath!(str);
                get_deps!(str);
                get_crates!(str);
                mkinclude!(str);
                 
            }}
mkmod!{time, { 
                getname!(time);
                getsrc!(time);
                getpath!(time);
                get_deps!(time);
                get_crates!(time);
                mkinclude!(time);
                 
            }}
mkmod!{wtf8, { 
                getname!(wtf8);
                getsrc!(wtf8);
                getpath!(wtf8);
                get_deps!(wtf8);
                get_crates!(wtf8);
                mkinclude!(wtf8);
                 
            }}
mkmod!{unicode, { 
                getname!(unicode);
                getsrc!(unicode);
                getpath!(unicode);
                get_deps!(unicode);
                get_crates!(unicode);
                mkinclude!(unicode);
                 
            }}
mkmod!{future, { 
                getname!(future);
                getsrc!(future);
                getpath!(future);
                get_deps!(future);
                get_crates!(future);
                mkinclude!(future);
                 
            }}
mkmod!{task, { 
                getname!(task);
                getsrc!(task);
                getpath!(task);
                get_deps!(task);
                get_crates!(task);
                mkinclude!(task);
                 
            }}
mkmod!{alloc, { 
                getname!(alloc);
                getsrc!(alloc);
                getpath!(alloc);
                get_deps!(alloc);
                get_crates!(alloc);
                mkinclude!(alloc);
                 
            }}
mkmod!{bool, { 
                getname!(bool);
                getsrc!(bool);
                getpath!(bool);
                get_deps!(bool);
                get_crates!(bool);
                mkinclude!(bool);
                 
            }}
mkmod!{escape, { 
                getname!(escape);
                getsrc!(escape);
                getpath!(escape);
                get_deps!(escape);
                get_crates!(escape);
                mkinclude!(escape);
                 
            }}
mkmod!{tuple, { 
                getname!(tuple);
                getsrc!(tuple);
                getpath!(tuple);
                get_deps!(tuple);
                get_crates!(tuple);
                mkinclude!(tuple);
                 
            }}
mkmod!{unit, { 
                getname!(unit);
                getsrc!(unit);
                getpath!(unit);
                get_deps!(unit);
                get_crates!(unit);
                mkinclude!(unit);
                 
            }}
mkmod!{primitive, { 
                getname!(primitive);
                getsrc!(primitive);
                getpath!(primitive);
                get_deps!(primitive);
                get_crates!(primitive);
                mkinclude!(primitive);
                 
            }}
mkmod!{core_arch, { 
                getname!(core_arch);
                getsrc!(core_arch);
                getpath!(core_arch);
                get_deps!(core_arch);
                get_crates!(core_arch);
                mkinclude!(core_arch);
                 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                 
            }}
mkmod!{core_simd, { 
                getname!(core_simd);
                getsrc!(core_simd);
                getpath!(core_simd);
                get_deps!(core_simd);
                get_crates!(core_simd);
                mkinclude!(core_simd);
                 
            }}
mkmod!{simd, { 
                getname!(simd);
                getsrc!(simd);
                getpath!(simd);
                get_deps!(simd);
                get_crates!(simd);
                mkinclude!(simd);
                mkuse!{# [unstable (feature = "portable_simd" , issue = "86656")] pub use crate :: core_simd :: simd :: * ;} 
            }}
mkitem!{include ! ("primitive_docs.rs") ;}