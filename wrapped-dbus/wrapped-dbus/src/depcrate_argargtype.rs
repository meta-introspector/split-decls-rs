// Generated macro for ArgType (enum)
macro_rules! Depcrate_argArgType {
() => {
// Module: crate::arg
// Provides: {"ArgType"}
// Dependencies: {}
# [doc = " Type of Argument"] # [doc = ""] # [doc = " use this to figure out, e g, which type of argument is at the current position of Iter."] # [repr (u8)] # [derive (Copy , Clone , Debug , Hash , Eq , PartialEq , Ord , PartialOrd)] pub enum ArgType { # [doc = " Dicts are Arrays of dict entries, so Dict types will have Array as ArgType."] Array = ffi :: DBUS_TYPE_ARRAY as u8 , # [doc = " Variant"] Variant = ffi :: DBUS_TYPE_VARIANT as u8 , # [doc = " bool"] Boolean = ffi :: DBUS_TYPE_BOOLEAN as u8 , # [doc = " Invalid arg type - this is also the ArgType returned when there are no more arguments available."] Invalid = ffi :: DBUS_TYPE_INVALID as u8 , # [doc = " String"] String = ffi :: DBUS_TYPE_STRING as u8 , # [doc = " Dict entry; you'll usually not encounter this one as dicts are arrays of dict entries."] DictEntry = ffi :: DBUS_TYPE_DICT_ENTRY as u8 , # [doc = " u8"] Byte = ffi :: DBUS_TYPE_BYTE as u8 , # [doc = " i16"] Int16 = ffi :: DBUS_TYPE_INT16 as u8 , # [doc = " u16"] UInt16 = ffi :: DBUS_TYPE_UINT16 as u8 , # [doc = " i32"] Int32 = ffi :: DBUS_TYPE_INT32 as u8 , # [doc = " u32"] UInt32 = ffi :: DBUS_TYPE_UINT32 as u8 , # [doc = " i64"] Int64 = ffi :: DBUS_TYPE_INT64 as u8 , # [doc = " u64"] UInt64 = ffi :: DBUS_TYPE_UINT64 as u8 , # [doc = " f64"] Double = ffi :: DBUS_TYPE_DOUBLE as u8 , # [doc = " File"] UnixFd = ffi :: DBUS_TYPE_UNIX_FD as u8 , # [doc = " Use tuples or Vec<Box<dyn RefArg>> to read/write structs."] Struct = ffi :: DBUS_TYPE_STRUCT as u8 , # [doc = " Path"] ObjectPath = ffi :: DBUS_TYPE_OBJECT_PATH as u8 , # [doc = " Signature"] Signature = ffi :: DBUS_TYPE_SIGNATURE as u8 , }
};
}
