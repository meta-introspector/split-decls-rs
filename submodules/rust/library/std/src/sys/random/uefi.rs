
macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { if bytes . is_empty () { return ; } if rng_protocol :: fill_bytes (bytes) { return ; } # [cfg (any (target_arch = "x86_64" , target_arch = "x86"))] if rdrand :: fill_bytes (bytes) { return ; } panic ! ("failed to generate random data") ; }
}
mkmod!{rng_protocol, { 
                getname!(rng_protocol);
                getsrc!(rng_protocol);
                getpath!(rng_protocol);
                get_deps!(rng_protocol);
                get_crates!(rng_protocol);
                mkinclude!(rng_protocol);
                mkuse!{use r_efi :: protocols :: rng ;}
mkuse!{use crate :: sys :: pal :: helpers ;}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub (crate) fn fill_bytes (bytes : & mut [u8]) -> bool { if let Ok (handles) = helpers :: locate_handles (rng :: PROTOCOL_GUID) { for handle in handles { if let Ok (protocol) = helpers :: open_protocol :: < rng :: Protocol > (handle , rng :: PROTOCOL_GUID) { let r = unsafe { ((* protocol . as_ptr ()) . get_rng) (protocol . as_ptr () , crate :: ptr :: null_mut () , bytes . len () , bytes . as_mut_ptr () ,) } ; if r . is_error () { continue ; } else { return true ; } } } } false }
} 
            }}
mkmod!{rdrand, { 
                getname!(rdrand);
                getsrc!(rdrand);
                getpath!(rdrand);
                get_deps!(rdrand);
                get_crates!(rdrand);
                mkinclude!(rdrand);
                mkitem!{cfg_select ! { target_arch = "x86_64" => { use crate :: arch :: x86_64 as arch ; use arch :: _rdrand64_step as rdrand_step ; type Word = u64 ; } target_arch = "x86" => { use crate :: arch :: x86 as arch ; use arch :: _rdrand32_step as rdrand_step ; type Word = u32 ; } }}
mkitem!{static RDRAND_GOOD : crate :: sync :: LazyLock < bool > = crate :: sync :: LazyLock :: new (is_rdrand_good) ;}
mkitem!{const RETRY_LIMIT : usize = 10 ;}

macro_rules! rdrand_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rdrand in module {}", module_path!());
    };
}

mkfn!{
    rdrand_introspect!();
    unsafe fn rdrand () -> Option < Word > { for _ in 0 .. RETRY_LIMIT { let mut val = 0 ; if unsafe { rdrand_step (& mut val) } == 1 { return Some (val) ; } } None }
}

macro_rules! self_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function self_test in module {}", module_path!());
    };
}

mkfn!{
    self_test_introspect!();
    unsafe fn self_test () -> bool { let mut prev = Word :: MAX ; let mut fails = 0 ; for _ in 0 .. 8 { match unsafe { rdrand () } { Some (val) if val == prev => fails += 1 , Some (val) => prev = val , None => return false , } ; } fails <= 2 }
}

macro_rules! is_rdrand_good_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_rdrand_good in module {}", module_path!());
    };
}

mkfn!{
    is_rdrand_good_introspect!();
    fn is_rdrand_good () -> bool { # [cfg (not (target_feature = "rdrand"))] { let cpuid0 = unsafe { arch :: __cpuid (0) } ; if cpuid0 . eax < 1 { return false ; } let cpuid1 = unsafe { arch :: __cpuid (1) } ; let vendor_id = [cpuid0 . ebx . to_le_bytes () , cpuid0 . edx . to_le_bytes () , cpuid0 . ecx . to_le_bytes ()] ; if vendor_id == [* b"Auth" , * b"enti" , * b"cAMD"] { let mut family = (cpuid1 . eax >> 8) & 0xF ; if family == 0xF { family += (cpuid1 . eax >> 20) & 0xFF ; } if family < 0x17 { return false ; } } const RDRAND_FLAG : u32 = 1 << 30 ; if cpuid1 . ecx & RDRAND_FLAG == 0 { return false ; } } unsafe { self_test () } }
}

macro_rules! rdrand_exact_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rdrand_exact in module {}", module_path!());
    };
}

mkfn!{
    rdrand_exact_introspect!();
    unsafe fn rdrand_exact (dest : & mut [u8]) -> Option < () > { let (chunks , tail) = dest . as_chunks_mut () ; for chunk in chunks { * chunk = unsafe { rdrand () } ? . to_ne_bytes () ; } let n = tail . len () ; if n > 0 { let src = unsafe { rdrand () } ? . to_ne_bytes () ; tail . copy_from_slice (& src [.. n]) ; } Some (()) }
}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub (crate) fn fill_bytes (bytes : & mut [u8]) -> bool { if * RDRAND_GOOD { unsafe { rdrand_exact (bytes) . is_some () } } else { false } }
} 
            }}