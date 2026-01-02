mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{fxsr, { 
                getname!(fxsr);
                getsrc!(fxsr);
                getpath!(fxsr);
                get_deps!(fxsr);
                get_crates!(fxsr);
                mkinclude!(fxsr);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: fxsr :: * ;}
mkmod!{sse, { 
                getname!(sse);
                getsrc!(sse);
                getpath!(sse);
                get_deps!(sse);
                get_crates!(sse);
                mkinclude!(sse);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sse :: * ;}
mkmod!{sse2, { 
                getname!(sse2);
                getsrc!(sse2);
                getpath!(sse2);
                get_deps!(sse2);
                get_crates!(sse2);
                mkinclude!(sse2);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sse2 :: * ;}
mkmod!{sse41, { 
                getname!(sse41);
                getsrc!(sse41);
                getpath!(sse41);
                get_deps!(sse41);
                get_crates!(sse41);
                mkinclude!(sse41);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sse41 :: * ;}
mkmod!{sse42, { 
                getname!(sse42);
                getsrc!(sse42);
                getpath!(sse42);
                get_deps!(sse42);
                get_crates!(sse42);
                mkinclude!(sse42);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sse42 :: * ;}
mkmod!{xsave, { 
                getname!(xsave);
                getsrc!(xsave);
                getpath!(xsave);
                get_deps!(xsave);
                get_crates!(xsave);
                mkinclude!(xsave);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: xsave :: * ;}
mkmod!{abm, { 
                getname!(abm);
                getsrc!(abm);
                getpath!(abm);
                get_deps!(abm);
                get_crates!(abm);
                mkinclude!(abm);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: abm :: * ;}
mkmod!{avx, { 
                getname!(avx);
                getsrc!(avx);
                getpath!(avx);
                get_deps!(avx);
                get_crates!(avx);
                mkinclude!(avx);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: avx :: * ;}
mkmod!{bmi, { 
                getname!(bmi);
                getsrc!(bmi);
                getpath!(bmi);
                get_deps!(bmi);
                get_crates!(bmi);
                mkinclude!(bmi);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: bmi :: * ;}
mkmod!{bmi2, { 
                getname!(bmi2);
                getsrc!(bmi2);
                getpath!(bmi2);
                get_deps!(bmi2);
                get_crates!(bmi2);
                mkinclude!(bmi2);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: bmi2 :: * ;}
mkmod!{tbm, { 
                getname!(tbm);
                getsrc!(tbm);
                getpath!(tbm);
                get_deps!(tbm);
                get_crates!(tbm);
                mkinclude!(tbm);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: tbm :: * ;}
mkmod!{avx512f, { 
                getname!(avx512f);
                getsrc!(avx512f);
                getpath!(avx512f);
                get_deps!(avx512f);
                get_crates!(avx512f);
                mkinclude!(avx512f);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512f :: * ;}
mkmod!{avx512bw, { 
                getname!(avx512bw);
                getsrc!(avx512bw);
                getpath!(avx512bw);
                get_deps!(avx512bw);
                get_crates!(avx512bw);
                mkinclude!(avx512bw);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512bw :: * ;}
mkmod!{bswap, { 
                getname!(bswap);
                getsrc!(bswap);
                getpath!(bswap);
                get_deps!(bswap);
                get_crates!(bswap);
                mkinclude!(bswap);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: bswap :: * ;}
mkmod!{rdrand, { 
                getname!(rdrand);
                getsrc!(rdrand);
                getpath!(rdrand);
                get_deps!(rdrand);
                get_crates!(rdrand);
                mkinclude!(rdrand);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: rdrand :: * ;}
mkmod!{cmpxchg16b, { 
                getname!(cmpxchg16b);
                getsrc!(cmpxchg16b);
                getpath!(cmpxchg16b);
                get_deps!(cmpxchg16b);
                get_crates!(cmpxchg16b);
                mkinclude!(cmpxchg16b);
                 
            }}
mkuse!{# [stable (feature = "cmpxchg16b_intrinsic" , since = "1.67.0")] pub use self :: cmpxchg16b :: * ;}
mkmod!{adx, { 
                getname!(adx);
                getsrc!(adx);
                getpath!(adx);
                get_deps!(adx);
                get_crates!(adx);
                mkinclude!(adx);
                 
            }}
mkuse!{# [stable (feature = "simd_x86_adx" , since = "1.33.0")] pub use self :: adx :: * ;}
mkmod!{bt, { 
                getname!(bt);
                getsrc!(bt);
                getpath!(bt);
                get_deps!(bt);
                get_crates!(bt);
                mkinclude!(bt);
                 
            }}
mkuse!{# [stable (feature = "simd_x86_bittest" , since = "1.55.0")] pub use self :: bt :: * ;}
mkmod!{avx512fp16, { 
                getname!(avx512fp16);
                getsrc!(avx512fp16);
                getpath!(avx512fp16);
                get_deps!(avx512fp16);
                get_crates!(avx512fp16);
                mkinclude!(avx512fp16);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_x86_avx512_f16" , issue = "127213")] pub use self :: avx512fp16 :: * ;}
mkmod!{amx, { 
                getname!(amx);
                getsrc!(amx);
                getpath!(amx);
                get_deps!(amx);
                get_crates!(amx);
                mkinclude!(amx);
                 
            }}
mkuse!{# [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub use self :: amx :: * ;}