mkmod!{core_simd, { 
                getname!(core_simd);
                getsrc!(core_simd);
                getpath!(core_simd);
                get_deps!(core_simd);
                get_crates!(core_simd);
                mkinclude!(core_simd);
                 
            }}
mkuse!{pub use self :: core_simd :: simd ;}