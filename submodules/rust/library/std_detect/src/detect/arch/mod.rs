mkmod!{x86, { 
                getname!(x86);
                getsrc!(x86);
                getpath!(x86);
                get_deps!(x86);
                get_crates!(x86);
                mkinclude!(x86);
                 
            }}
mkmod!{arm, { 
                getname!(arm);
                getsrc!(arm);
                getpath!(arm);
                get_deps!(arm);
                get_crates!(arm);
                mkinclude!(arm);
                 
            }}
mkmod!{aarch64, { 
                getname!(aarch64);
                getsrc!(aarch64);
                getpath!(aarch64);
                get_deps!(aarch64);
                get_crates!(aarch64);
                mkinclude!(aarch64);
                 
            }}
mkmod!{riscv, { 
                getname!(riscv);
                getsrc!(riscv);
                getpath!(riscv);
                get_deps!(riscv);
                get_crates!(riscv);
                mkinclude!(riscv);
                 
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
mkmod!{mips, { 
                getname!(mips);
                getsrc!(mips);
                getpath!(mips);
                get_deps!(mips);
                get_crates!(mips);
                mkinclude!(mips);
                 
            }}
mkmod!{mips64, { 
                getname!(mips64);
                getsrc!(mips64);
                getpath!(mips64);
                get_deps!(mips64);
                get_crates!(mips64);
                mkinclude!(mips64);
                 
            }}
mkmod!{loongarch, { 
                getname!(loongarch);
                getsrc!(loongarch);
                getpath!(loongarch);
                get_deps!(loongarch);
                get_crates!(loongarch);
                mkinclude!(loongarch);
                 
            }}
mkmod!{s390x, { 
                getname!(s390x);
                getsrc!(s390x);
                getpath!(s390x);
                get_deps!(s390x);
                get_crates!(s390x);
                mkinclude!(s390x);
                 
            }}
mkitem!{cfg_select ! { any (target_arch = "x86" , target_arch = "x86_64") => { # [stable (feature = "simd_x86" , since = "1.27.0")] pub use x86 ::*; } target_arch = "arm" => { # [unstable (feature = "stdarch_arm_feature_detection" , issue = "111190")] pub use arm ::*; } any (target_arch = "aarch64" , target_arch = "arm64ec") => { # [stable (feature = "simd_aarch64" , since = "1.60.0")] pub use aarch64 ::*; } any (target_arch = "riscv32" , target_arch = "riscv64") => { # [unstable (feature = "stdarch_riscv_feature_detection" , issue = "111192")] pub use riscv ::*; } target_arch = "powerpc" => { # [unstable (feature = "stdarch_powerpc_feature_detection" , issue = "111191")] pub use powerpc ::*; } target_arch = "powerpc64" => { # [unstable (feature = "stdarch_powerpc_feature_detection" , issue = "111191")] pub use powerpc64 ::*; } target_arch = "mips" => { # [unstable (feature = "stdarch_mips_feature_detection" , issue = "111188")] pub use mips ::*; } target_arch = "mips64" => { # [unstable (feature = "stdarch_mips_feature_detection" , issue = "111188")] pub use mips64 ::*; } any (target_arch = "loongarch32" , target_arch = "loongarch64") => { # [stable (feature = "stdarch_loongarch_feature" , since = "1.89.0")] pub use loongarch ::*; } target_arch = "s390x" => { # [unstable (feature = "stdarch_s390x_feature_detection" , issue = "135413")] pub use s390x ::*; } _ => { # [doc (hidden)] pub (crate) enum Feature { Null } # [doc (hidden)] # [unstable (feature = "stdarch_internal" , issue = "none")] pub mod __is_feature_detected { } impl Feature { # [doc (hidden)] pub (crate) fn from_str (_s : & str) -> Result < Feature , () > { Err (()) } # [doc (hidden)] pub (crate) fn to_str (self) -> &'static str { "" } } } }}