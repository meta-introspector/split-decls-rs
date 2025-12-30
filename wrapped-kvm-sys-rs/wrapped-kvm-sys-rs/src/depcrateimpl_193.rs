// Generated macro for impl_193 (impl)
macro_rules! Depcrateimpl_193 {
() => {
// Module: crate
// Provides: {"impl_193"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] impl System { # [doc = " Get CPUID features supported by this host"] pub fn get_supported_cpuid (& self) -> Result < CpuidHandle > { let mut nent = CPUID_ENTRIES ; loop { let mut c = CpuidHandle :: new (nent) ; let err = unsafe { kvm_get_supported_cpuid (self . fd . as_raw_fd () , c . deref_mut ()) } ; if err != 0 { if errno () == Errno (E2BIG) { nent *= 2 ; continue ; } else if errno () == Errno (ENOMEM) { nent = c . nent ; continue ; } else { return Err (Error :: last_os_error ()) ; } } else { return Ok (c) ; } } } }
};
}
