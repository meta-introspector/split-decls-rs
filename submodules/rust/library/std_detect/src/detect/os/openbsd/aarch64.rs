mkuse!{use core :: mem :: MaybeUninit ;}
mkuse!{use core :: ptr ;}
mkuse!{use crate :: detect :: cache ;}
mkitem!{const CPU_ID_AA64ISAR0 : libc :: c_int = 2 ;}
mkitem!{const CPU_ID_AA64ISAR1 : libc :: c_int = 3 ;}
mkitem!{const CPU_ID_AA64MMFR2 : libc :: c_int = 7 ;}
mkitem!{const CPU_ID_AA64PFR0 : libc :: c_int = 8 ;}

macro_rules! detect_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_features in module {}", module_path!());
    };
}

mkfn!{
    detect_features_introspect!();
    # [doc = " Try to read the features from the system registers."] pub (crate) fn detect_features () -> cache :: Initializer { let aa64isar0 = sysctl64 (& [libc :: CTL_MACHDEP , CPU_ID_AA64ISAR0]) . unwrap_or (0) ; let aa64isar1 = sysctl64 (& [libc :: CTL_MACHDEP , CPU_ID_AA64ISAR1]) . unwrap_or (0) ; let aa64mmfr2 = sysctl64 (& [libc :: CTL_MACHDEP , CPU_ID_AA64MMFR2]) . unwrap_or (0) ; let aa64pfr0 = sysctl64 (& [libc :: CTL_MACHDEP , CPU_ID_AA64PFR0]) ; super :: aarch64 :: parse_system_registers (aa64isar0 , aa64isar1 , aa64mmfr2 , aa64pfr0) }
}

macro_rules! sysctl64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sysctl64 in module {}", module_path!());
    };
}

mkfn!{
    sysctl64_introspect!();
    # [inline] fn sysctl64 (mib : & [libc :: c_int]) -> Option < u64 > { const OUT_LEN : libc :: size_t = core :: mem :: size_of :: < u64 > () ; let mut out = MaybeUninit :: < u64 > :: uninit () ; let mut out_len = OUT_LEN ; let res = unsafe { libc :: sysctl (mib . as_ptr () , mib . len () as libc :: c_uint , out . as_mut_ptr () as * mut libc :: c_void , & mut out_len , ptr :: null_mut () , 0 ,) } ; if res == - 1 || out_len != OUT_LEN { return None ; } Some (unsafe { out . assume_init () }) }
}