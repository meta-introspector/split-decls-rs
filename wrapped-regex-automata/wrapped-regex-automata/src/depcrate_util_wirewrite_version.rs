// Generated macro for write_version (function)
macro_rules! Depcrate_util_wirewrite_version {
() => {
// Module: crate::util::wire
// Provides: {"write_version"}
// Dependencies: {}
# [doc = " Writes the given version number to the beginning of the given slice."] # [doc = ""] # [doc = " This is useful for writing into the header of a serialized object. It can"] # [doc = " be read during deserialization as a sanity check to ensure that the library"] # [doc = " code supports the format of the serialized object."] # [doc = ""] # [doc = " Upon success, the total number of bytes written is returned."] pub (crate) fn write_version < E : Endian > (version : u32 , dst : & mut [u8] ,) -> Result < usize , SerializeError > { let nwrite = write_version_len () ; if dst . len () < nwrite { return Err (SerializeError :: buffer_too_small ("version number")) ; } E :: write_u32 (version , dst) ; Ok (nwrite) }
};
}
