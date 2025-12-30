// Generated macro for impl_197 (impl)
macro_rules! Depcrateimpl_197 {
() => {
// Module: crate
// Provides: {"impl_197"}
// Dependencies: {}
impl < 'a > Vcpu < 'a > { # [doc = " Create a `Vcpu` on the specified `VirtualMachine`"] pub fn create (vm : & 'a mut VirtualMachine < 'a >) -> Result < Self > { if vm . num_vcpus >= vm . sys . max_vcpus () { return Err (Error :: new (ErrorKind :: AlreadyExists , "Would exceed max_vcpus")) ; } else if vm . num_vcpus >= vm . sys . recommended_vcpus () { warn ! ("Exceeding recommended_vcpus") ; } let fd = unsafe { File :: from_raw_fd (kvm_create_vcpu (vm . fd . as_raw_fd () , vm . num_vcpus as c_int)) } ; vm . num_vcpus += 1 ; let mmap_size = vm . sys . get_vcpu_mmap_size () ; let m = try ! (Mmap :: open_with_offset (& fd , Protection :: ReadWrite , 0 , mmap_size)) ; Ok (Vcpu { fd : fd , vm : vm , mmap : m , }) } # [doc = " Run the `Vcpu`"] pub unsafe fn run (& mut self) -> Result < Run > { let ret = kvm_run (self . fd . as_raw_fd ()) ; if ret == 0 { Ok (* (self . mmap . mut_ptr () as * mut Run)) } else { Err (Error :: last_os_error ()) } } # [doc = " Get registers"] pub fn get_regs (& self) -> Result < Regs > { let mut regs = Regs :: default () ; let ret = unsafe { kvm_get_regs (self . fd . as_raw_fd () , & mut regs) } ; if ret == 0 { Ok (regs) } else { Err (Error :: last_os_error ()) } } # [doc = " Set registers"] pub fn set_regs (& mut self , regs : & Regs) -> Result < () > { let ret = unsafe { kvm_set_regs (self . fd . as_raw_fd () , regs) } ; if ret == 0 { Ok (()) } else { Err (Error :: last_os_error ()) } } }
};
}
