// Generated macro for HostRepr (enum)
macro_rules! Depcrate_buildHostRepr {
() => {
// Module: crate::build
// Provides: {"HostRepr"}
// Dependencies: {}
# [doc = " Host representation."] # [derive (Debug , Clone , Copy)] enum HostRepr < 'a > { # [doc = " Direct string representation."] String (& 'a str) , # [cfg (feature = "std")] # [doc = " Dedicated IP address type."] IpAddr (std :: net :: IpAddr) , }
};
}
