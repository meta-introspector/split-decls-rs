mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{riscv_shared, { 
                getname!(riscv_shared);
                getsrc!(riscv_shared);
                getpath!(riscv_shared);
                get_deps!(riscv_shared);
                get_crates!(riscv_shared);
                mkinclude!(riscv_shared);
                 
            }}
mkmod!{arm_shared, { 
                getname!(arm_shared);
                getsrc!(arm_shared);
                getpath!(arm_shared);
                get_deps!(arm_shared);
                get_crates!(arm_shared);
                mkinclude!(arm_shared);
                 
            }}
mkmod!{loongarch_shared, { 
                getname!(loongarch_shared);
                getsrc!(loongarch_shared);
                getpath!(loongarch_shared);
                get_deps!(loongarch_shared);
                get_crates!(loongarch_shared);
                mkinclude!(loongarch_shared);
                 
            }}
mkmod!{simd, { 
                getname!(simd);
                getsrc!(simd);
                getpath!(simd);
                get_deps!(simd);
                get_crates!(simd);
                mkinclude!(simd);
                 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkmod!{x86, { 
                getname!(x86);
                getsrc!(x86);
                getpath!(x86);
                get_deps!(x86);
                get_crates!(x86);
                mkinclude!(x86);
                mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use crate :: core_arch :: x86 :: * ;} 
            }}
mkmod!{x86_64, { 
                getname!(x86_64);
                getsrc!(x86_64);
                getpath!(x86_64);
                get_deps!(x86_64);
                get_crates!(x86_64);
                mkinclude!(x86_64);
                mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use crate :: core_arch :: x86 :: * ;}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use crate :: core_arch :: x86_64 :: * ;} 
            }}
mkmod!{arm, { 
                getname!(arm);
                getsrc!(arm);
                getpath!(arm);
                get_deps!(arm);
                get_crates!(arm);
                mkinclude!(arm);
                mkuse!{# [unstable (feature = "stdarch_arm_neon_intrinsics" , issue = "111800")] pub use crate :: core_arch :: arm :: * ;} 
            }}
mkmod!{aarch64, { 
                getname!(aarch64);
                getsrc!(aarch64);
                getpath!(aarch64);
                get_deps!(aarch64);
                get_crates!(aarch64);
                mkinclude!(aarch64);
                mkuse!{# [stable (feature = "neon_intrinsics" , since = "1.59.0")] pub use crate :: core_arch :: aarch64 :: * ;} 
            }}
mkmod!{riscv32, { 
                getname!(riscv32);
                getsrc!(riscv32);
                getpath!(riscv32);
                get_deps!(riscv32);
                get_crates!(riscv32);
                mkinclude!(riscv32);
                mkuse!{pub use crate :: core_arch :: riscv_shared :: * ;}
mkuse!{pub use crate :: core_arch :: riscv32 :: * ;} 
            }}
mkmod!{riscv64, { 
                getname!(riscv64);
                getsrc!(riscv64);
                getpath!(riscv64);
                get_deps!(riscv64);
                get_crates!(riscv64);
                mkinclude!(riscv64);
                mkuse!{pub use crate :: core_arch :: riscv64 :: * ;}
mkuse!{pub use crate :: core_arch :: riscv_shared :: * ;} 
            }}
mkmod!{wasm32, { 
                getname!(wasm32);
                getsrc!(wasm32);
                getpath!(wasm32);
                get_deps!(wasm32);
                get_crates!(wasm32);
                mkinclude!(wasm32);
                mkuse!{# [stable (feature = "simd_wasm32" , since = "1.33.0")] pub use crate :: core_arch :: wasm32 :: * ;} 
            }}
mkmod!{wasm64, { 
                getname!(wasm64);
                getsrc!(wasm64);
                getpath!(wasm64);
                get_deps!(wasm64);
                get_crates!(wasm64);
                mkinclude!(wasm64);
                mkuse!{# [unstable (feature = "simd_wasm64" , issue = "90599")] pub use crate :: core_arch :: wasm32 :: * ;} 
            }}
mkmod!{wasm, { 
                getname!(wasm);
                getsrc!(wasm);
                getpath!(wasm);
                get_deps!(wasm);
                get_crates!(wasm);
                mkinclude!(wasm);
                mkuse!{# [unstable (feature = "simd_wasm64" , issue = "90599")] pub use crate :: core_arch :: wasm32 :: * ;} 
            }}
mkmod!{mips, { 
                getname!(mips);
                getsrc!(mips);
                getpath!(mips);
                get_deps!(mips);
                get_crates!(mips);
                mkinclude!(mips);
                mkuse!{pub use crate :: core_arch :: mips :: * ;} 
            }}
mkmod!{mips64, { 
                getname!(mips64);
                getsrc!(mips64);
                getpath!(mips64);
                get_deps!(mips64);
                get_crates!(mips64);
                mkinclude!(mips64);
                mkuse!{pub use crate :: core_arch :: mips :: * ;} 
            }}
mkmod!{powerpc, { 
                getname!(powerpc);
                getsrc!(powerpc);
                getpath!(powerpc);
                get_deps!(powerpc);
                get_crates!(powerpc);
                mkinclude!(powerpc);
                mkuse!{pub use crate :: core_arch :: powerpc :: * ;} 
            }}
mkmod!{powerpc64, { 
                getname!(powerpc64);
                getsrc!(powerpc64);
                getpath!(powerpc64);
                get_deps!(powerpc64);
                get_crates!(powerpc64);
                mkinclude!(powerpc64);
                mkuse!{pub use crate :: core_arch :: powerpc64 :: * ;} 
            }}
mkmod!{nvptx, { 
                getname!(nvptx);
                getsrc!(nvptx);
                getpath!(nvptx);
                get_deps!(nvptx);
                get_crates!(nvptx);
                mkinclude!(nvptx);
                mkuse!{pub use crate :: core_arch :: nvptx :: * ;} 
            }}
mkmod!{loongarch32, { 
                getname!(loongarch32);
                getsrc!(loongarch32);
                getpath!(loongarch32);
                get_deps!(loongarch32);
                get_crates!(loongarch32);
                mkinclude!(loongarch32);
                mkuse!{pub use crate :: core_arch :: loongarch_shared :: * ;}
mkuse!{pub use crate :: core_arch :: loongarch32 :: * ;} 
            }}
mkmod!{loongarch64, { 
                getname!(loongarch64);
                getsrc!(loongarch64);
                getpath!(loongarch64);
                get_deps!(loongarch64);
                get_crates!(loongarch64);
                mkinclude!(loongarch64);
                mkuse!{pub use crate :: core_arch :: loongarch_shared :: * ;}
mkuse!{pub use crate :: core_arch :: loongarch64 :: * ;} 
            }}
mkmod!{s390x, { 
                getname!(s390x);
                getsrc!(s390x);
                getpath!(s390x);
                get_deps!(s390x);
                get_crates!(s390x);
                mkinclude!(s390x);
                mkuse!{pub use crate :: core_arch :: s390x :: * ;} 
            }} 
            }}
mkmod!{x86, { 
                getname!(x86);
                getsrc!(x86);
                getpath!(x86);
                get_deps!(x86);
                get_crates!(x86);
                mkinclude!(x86);
                 
            }}
mkmod!{x86_64, { 
                getname!(x86_64);
                getsrc!(x86_64);
                getpath!(x86_64);
                get_deps!(x86_64);
                get_crates!(x86_64);
                mkinclude!(x86_64);
                 
            }}
mkmod!{aarch64, { 
                getname!(aarch64);
                getsrc!(aarch64);
                getpath!(aarch64);
                get_deps!(aarch64);
                get_crates!(aarch64);
                mkinclude!(aarch64);
                 
            }}
mkmod!{arm, { 
                getname!(arm);
                getsrc!(arm);
                getpath!(arm);
                get_deps!(arm);
                get_crates!(arm);
                mkinclude!(arm);
                 
            }}
mkmod!{riscv32, { 
                getname!(riscv32);
                getsrc!(riscv32);
                getpath!(riscv32);
                get_deps!(riscv32);
                get_crates!(riscv32);
                mkinclude!(riscv32);
                 
            }}
mkmod!{riscv64, { 
                getname!(riscv64);
                getsrc!(riscv64);
                getpath!(riscv64);
                get_deps!(riscv64);
                get_crates!(riscv64);
                mkinclude!(riscv64);
                 
            }}
mkmod!{wasm32, { 
                getname!(wasm32);
                getsrc!(wasm32);
                getpath!(wasm32);
                get_deps!(wasm32);
                get_crates!(wasm32);
                mkinclude!(wasm32);
                 
            }}
mkmod!{mips, { 
                getname!(mips);
                getsrc!(mips);
                getpath!(mips);
                get_deps!(mips);
                get_crates!(mips);
                mkinclude!(mips);
                 
            }}
mkmod!{powerpc, { 
                getname!(powerpc);
                getsrc!(powerpc);
                getpath!(powerpc);
                get_deps!(powerpc);
                get_crates!(powerpc);
                mkinclude!(powerpc);
                 
            }}
mkmod!{powerpc64, { 
                getname!(powerpc64);
                getsrc!(powerpc64);
                getpath!(powerpc64);
                get_deps!(powerpc64);
                get_crates!(powerpc64);
                mkinclude!(powerpc64);
                 
            }}
mkmod!{nvptx, { 
                getname!(nvptx);
                getsrc!(nvptx);
                getpath!(nvptx);
                get_deps!(nvptx);
                get_crates!(nvptx);
                mkinclude!(nvptx);
                 
            }}
mkmod!{loongarch32, { 
                getname!(loongarch32);
                getsrc!(loongarch32);
                getpath!(loongarch32);
                get_deps!(loongarch32);
                get_crates!(loongarch32);
                mkinclude!(loongarch32);
                 
            }}
mkmod!{loongarch64, { 
                getname!(loongarch64);
                getsrc!(loongarch64);
                getpath!(loongarch64);
                get_deps!(loongarch64);
                get_crates!(loongarch64);
                mkinclude!(loongarch64);
                 
            }}
mkmod!{s390x, { 
                getname!(s390x);
                getsrc!(s390x);
                getpath!(s390x);
                get_deps!(s390x);
                get_crates!(s390x);
                mkinclude!(s390x);
                 
            }}