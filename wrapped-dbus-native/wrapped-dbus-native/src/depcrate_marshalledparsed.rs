// Generated macro for Parsed (enum)
macro_rules! Depcrate_marshalledParsed {
() => {
// Module: crate::marshalled
// Provides: {"Parsed"}
// Dependencies: {}
# [non_exhaustive] # [derive (Debug , Clone)] pub enum Parsed < 'a > { # [doc = " A D-Bus array requires all elements to be of the same type."] Array (Array < 'a >) , # [doc = " A D-Bus dictionary requires all keys and all values to be of the same type."] Dict (Dict < 'a >) , # [doc = " A D-Bus struct is a list of values of different types."] Struct (Multi < 'a >) , # [doc = " A D-Bus variant is a wrapper around another type, which"] # [doc = " can be of any valid D-Bus type."] Variant (Single < 'a >) , # [doc = " A D-Bus object path."] ObjectPath (& 'a dbus_strings :: ObjectPath) , # [doc = " A D-Bus signature."] Signature (& 'a SignatureMulti) , # [doc = " A D-Bus String."] String (& 'a DBusStr) , # [doc = " A D-Bus boolean type."] Boolean (bool) , # [doc = " A D-Bus unsigned 8 bit type."] Byte (u8) , # [doc = " A D-Bus signed 16 bit type."] Int16 (i16) , # [doc = " A D-Bus signed 32 bit type."] Int32 (i32) , # [doc = " A D-Bus signed 64 bit type."] Int64 (i64) , # [doc = " A D-Bus unsigned 16 bit type."] UInt16 (u16) , # [doc = " A D-Bus unsigned 32 bit type."] UInt32 (u32) , # [doc = " A D-Bus unsigned 64 bit type."] UInt64 (u64) , # [doc = " A D-Bus IEEE-754 double-precision floating point type."] Double (f64) , # [doc = " D-Bus allows for sending file descriptors, which can be used to"] # [doc = " set up SHM, unix pipes, or other communication channels."] # [doc = ""] # [doc = " The usize is an index that can need to be used with the message to get the actual file descriptor out."] UnixFd (usize) , }
};
}
