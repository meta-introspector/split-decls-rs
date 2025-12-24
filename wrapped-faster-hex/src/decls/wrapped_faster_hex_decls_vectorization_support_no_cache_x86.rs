use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cold]
fn vectorization_support_no_cache_x86() -> Vectorization {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::__cpuid_count;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::__cpuid_count;
    if cfg!(target_env = "sgx") || !cfg!(target_feature = "sse") {
        return Vectorization::None;
    }
    let proc_info_ecx = unsafe { __cpuid_count(1, 0) }.ecx;
    let have_sse4 = (proc_info_ecx >> 19) & 1 == 1;
    if !have_sse4 {
        return Vectorization::None;
    }
    let have_xsave = (proc_info_ecx >> 26) & 1 == 1;
    let have_osxsave = (proc_info_ecx >> 27) & 1 == 1;
    let have_avx = (proc_info_ecx >> 28) & 1 == 1;
    if have_xsave && have_osxsave && have_avx {
        if unsafe { avx2_support_no_cache_x86() } {
            return Vectorization::AVX2;
        }
    }
    Vectorization::SSE41
}
