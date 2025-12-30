// Generated macro for impl_196 (impl)
macro_rules! Depcrateimpl_196 {
() => {
// Module: crate
// Provides: {"impl_196"}
// Dependencies: {}
impl < 'a > VirtualMachine < 'a > { # [doc = " Create a `VirtualMachine`"] pub fn create (s : & 'a System) -> Result < Self > { let f = unsafe { kvm_create_vm (s . fd . as_raw_fd () , 0) } ; if f == - 1 { return Err (Error :: last_os_error ()) ; } let check_extension = s . check_capability (Capability :: CheckExtensionVm) != 0 ; Ok (VirtualMachine { fd : unsafe { File :: from_raw_fd (f) } , sys : s , mem_slots : Vec :: new () , num_vcpus : 0 , check_extension : check_extension , }) } # [doc = " Check for a capability on this `VirtualMachine`"] pub fn check_capability (& mut self , cap : Capability) -> i32 { if self . check_extension { unsafe { kvm_check_extension (self . fd . as_raw_fd () , cap as c_int) } } else { self . sys . check_capability (cap) } } # [doc = " Establish a guest memory mapping."] # [doc = ""] # [doc = " The slice specified by `user_addr` is mapped at `phys_addr`. Flags is"] # [doc = " the bitwise or of `MEM_LOG_DIRTY_PAGES` and/or `MEM_READONLY`."] pub fn set_user_memory_region (& mut self , phys_addr : u64 , user_addr : & 'a mut [u8] , flags : u32) -> Result < () > { let slot = self . mem_slots . len () ; let region = UserspaceMemoryRegion { slot : slot as u32 , flags : flags , guest_phys_addr : phys_addr , memory_size : user_addr . len () as u64 , userspace_addr : user_addr . as_mut_ptr () as u64 , } ; let ret = unsafe { kvm_set_user_memory_region (self . fd . as_raw_fd () , & region) } ; if ret == 0 { self . mem_slots . push (user_addr) ; Ok (()) } else { Err (Error :: new (ErrorKind :: Other , "Unknown Error")) } } }
};
}
