// Generated macro for VirtualMachine (struct)
macro_rules! DepcrateVirtualMachine {
() => {
// Module: crate
// Provides: {"VirtualMachine"}
// Dependencies: {}
# [doc = " A Virtual Machine."] # [doc = ""] # [doc = " This allows the creation of `Vcpu`s and establishing memory mappings"] # [derive (Debug)] pub struct VirtualMachine < 'a > { fd : File , sys : & 'a System , mem_slots : Vec < & 'a mut [u8] > , num_vcpus : u32 , check_extension : bool , }
};
}
