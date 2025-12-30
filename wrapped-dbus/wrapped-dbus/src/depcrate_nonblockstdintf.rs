// Generated macro for stdintf (module)
macro_rules! Depcrate_nonblockstdintf {
() => {
// Module: crate::nonblock
// Provides: {"stdintf"}
// Dependencies: {}
# [doc = " This module contains some standard interfaces and an easy way to call them."] # [doc = ""] # [doc = " See the [D-Bus specification](https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces) for more information about these standard interfaces."] # [doc = ""] # [doc = " The code was created by dbus-codegen."] pub mod stdintf { # [allow (missing_docs)] pub mod org_freedesktop_dbus { pub use super :: super :: generated_org_freedesktop_standard_interfaces :: * ; # [allow (unused_imports)] pub (crate) use super :: super :: generated_org_freedesktop_dbus :: * ; # [derive (Debug , PartialEq , Eq , Copy , Clone)] pub enum RequestNameReply { PrimaryOwner = 1 , InQueue = 2 , Exists = 3 , AlreadyOwner = 4 , } # [derive (Debug , PartialEq , Eq , Copy , Clone)] pub enum ReleaseNameReply { Released = 1 , NonExistent = 2 , NotOwner = 3 , } } }
};
}
