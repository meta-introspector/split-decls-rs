// Generated macro for mtu (function)
macro_rules! Depcrate_drivers_netmtu {
() => {
// Module: crate::drivers::net
// Provides: {"mtu"}
// Dependencies: {}
# [doc = " Determines the MTU that should be used as configured by crate features"] # [doc = " or environment variables."] # [cfg (any (all (target_arch = "riscv64" , feature = "gem-net" , not (feature = "pci")) , feature = "rtl8139" , feature = "virtio-net" ,))] pub (crate) fn mtu () -> u16 { use core :: str :: FromStr ; const DEFAULT_IP_MTU : u16 = 1500 ; # [doc = " Default MTU to use."] # [doc = ""] # [doc = " This is 1500 IP MTU and a 14-byte ethernet header."] const DEFAULT_MTU : u16 = DEFAULT_IP_MTU + 14 ; if let Some (my_mtu) = hermit_var ! ("HERMIT_MTU") { u16 :: from_str (& my_mtu) . unwrap () } else { DEFAULT_MTU } }
};
}
