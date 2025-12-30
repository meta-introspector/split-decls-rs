// Generated macro for impl_191 (impl)
macro_rules! Depcrateimpl_191 {
() => {
// Module: crate
// Provides: {"impl_191"}
// Dependencies: {}
impl System { # [doc = " Initialize the KVM system"] pub fn initialize () -> Result < Self > { let f = try ! (OpenOptions :: new () . read (true) . write (true) . open ("/dev/kvm")) ; let vers = unsafe { kvm_get_api_version (f . as_raw_fd ()) } ; if vers == API_VERSION { Ok (System { fd : f }) } else { Err (Error :: new (ErrorKind :: NotFound , "Unexpected API Version")) } } # [doc = " Check for the existence of a capability."] # [doc = ""] # [doc = " Where possible use the associated function on a `VirtualMachine` rather"] # [doc = " than the `System` since `VirtualMachine`s may have different"] # [doc = " capabilities"] pub fn check_capability (& self , cap : Capability) -> i32 { unsafe { kvm_check_extension (self . fd . as_raw_fd () , cap as c_int) } } # [doc = " Recommended maximum number of `Vcpu`s"] pub fn recommended_vcpus (& self) -> u32 { let r = self . check_capability (Capability :: NrVcpus) ; if r != 0 { r as u32 } else { 4 } } # [doc = " Maximum number of `Vcpu`s"] pub fn max_vcpus (& self) -> u32 { let r = self . check_capability (Capability :: MaxVcpus) ; if r != 0 { r as u32 } else { self . recommended_vcpus () } } fn get_vcpu_mmap_size (& self) -> usize { let ret = unsafe { kvm_get_vcpu_mmap_size (self . fd . as_raw_fd ()) } ; assert ! (ret > 0 && ret as usize >= mem :: size_of ::< Run > ()) ; ret as usize } }
};
}
