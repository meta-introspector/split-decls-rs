// Generated macro for NameFlag (enum)
macro_rules! Depcrate_ffidispNameFlag {
() => {
// Module: crate::ffidisp
// Provides: {"NameFlag"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Copy , Clone)] # [doc = " Flags to use for Connection::register_name."] # [doc = ""] # [doc = " More than one flag can be specified, if so just add their values."] pub enum NameFlag { # [doc = " Allow another service to become the primary owner if requested"] AllowReplacement = ffi :: DBUS_NAME_FLAG_ALLOW_REPLACEMENT as isize , # [doc = " Request to replace the current primary owner"] ReplaceExisting = ffi :: DBUS_NAME_FLAG_REPLACE_EXISTING as isize , # [doc = " If we can not become the primary owner do not place us in the queue"] DoNotQueue = ffi :: DBUS_NAME_FLAG_DO_NOT_QUEUE as isize , }
};
}
