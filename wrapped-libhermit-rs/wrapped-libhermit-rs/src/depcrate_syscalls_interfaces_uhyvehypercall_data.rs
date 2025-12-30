// Generated macro for hypercall_data (function)
macro_rules! Depcrate_syscalls_interfaces_uhyvehypercall_data {
() => {
// Module: crate::syscalls::interfaces::uhyve
// Provides: {"hypercall_data"}
// Dependencies: {}
# [doc = " calculates the hypercall data argument"] # [inline] fn hypercall_data (hypercall : & Hypercall < '_ >) -> u64 { match hypercall { Hypercall :: Cmdsize (data) => data_addr (* data) , Hypercall :: Cmdval (data) => data_addr (* data) , Hypercall :: Exit (data) => data_addr (* data) , Hypercall :: FileClose (data) => data_addr (* data) , Hypercall :: FileLseek (data) => data_addr (* data) , Hypercall :: FileOpen (data) => data_addr (* data) , Hypercall :: FileRead (data) => data_addr (* data) , Hypercall :: FileUnlink (data) => data_addr (* data) , Hypercall :: FileWrite (data) => data_addr (* data) , Hypercall :: SerialWriteBuffer (data) => data_addr (* data) , Hypercall :: SerialWriteByte (byte) => u64 :: from (* byte) , h => todo ! ("unimplemented hypercall {h:?}") , } }
};
}
