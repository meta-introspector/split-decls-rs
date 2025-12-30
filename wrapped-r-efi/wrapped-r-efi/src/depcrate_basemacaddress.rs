// Generated macro for MacAddress (struct)
macro_rules! Depcrate_baseMacAddress {
() => {
// Module: crate::base
// Provides: {"MacAddress"}
// Dependencies: {}
# [doc = " Network MAC Address"] # [doc = ""] # [doc = " This type encapsulates a single networking media access control address"] # [doc = " (MAC). It is a simple 32 bytes buffer with no special alignment. Note that"] # [doc = " no comparison function are defined by default, since trailing bytes of the"] # [doc = " address might be random."] # [doc = ""] # [doc = " The interpretation of the content differs depending on the protocol it is"] # [doc = " used with. See each documentation for details. In most cases this contains"] # [doc = " an Ethernet address."] # [repr (C)] # [derive (Clone , Copy , Debug)] # [derive (Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct MacAddress { pub addr : [u8 ; 32] , }
};
}
