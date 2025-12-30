// Generated macro for impl_198 (impl)
macro_rules! Depcrateimpl_198 {
() => {
// Module: crate
// Provides: {"impl_198"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] impl < 'a > Vcpu < 'a > { # [doc = " Set the response to the CPUID instruction"] pub fn set_cpuid2 (& mut self , cpuid : & mut Cpuid2) -> Result < () > { let ptr : * mut Cpuid2 = cpuid ; let ret = unsafe { kvm_set_cpuid2 (self . fd . as_raw_fd () , ptr) } ; if ret == 0 { Ok (()) } else { Err (Error :: last_os_error ()) } } # [doc = " Get special registers"] pub fn get_sregs (& self) -> Result < Sregs > { let mut sregs = Sregs :: default () ; let ret = unsafe { kvm_get_sregs (self . fd . as_raw_fd () , & mut sregs) } ; if ret == 0 { Ok (sregs) } else { Err (Error :: last_os_error ()) } } # [doc = " Set special registers"] pub fn set_sregs (& mut self , sregs : & Sregs) -> Result < () > { let ret = unsafe { kvm_set_sregs (self . fd . as_raw_fd () , sregs) } ; if ret == 0 { Ok (()) } else { Err (Error :: last_os_error ()) } } }
};
}
