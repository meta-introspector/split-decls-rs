// Generated macro for WritableBuffer (trait)
macro_rules! Depcrate_write_utilWritableBuffer {
() => {
// Module: crate::write::util
// Provides: {"WritableBuffer"}
// Dependencies: {}
# [doc = " Trait for writable buffer."] # [allow (clippy :: len_without_is_empty)] pub trait WritableBuffer { # [doc = " Returns position/offset for data to be written at."] # [doc = ""] # [doc = " Should only be used in debug assertions"] fn len (& self) -> usize ; # [doc = " Reserves specified number of bytes in the buffer."] # [doc = ""] # [doc = " This will be called exactly once before writing anything to the buffer,"] # [doc = " and the given size is the exact total number of bytes that will be written."] fn reserve (& mut self , size : usize) -> Result < () , () > ; # [doc = " Writes zero bytes at the end of the buffer until the buffer"] # [doc = " has the specified length."] fn resize (& mut self , new_len : usize) ; # [doc = " Writes the specified slice of bytes at the end of the buffer."] fn write_bytes (& mut self , val : & [u8]) ; # [doc = " Writes the specified `Pod` type at the end of the buffer."] fn write_pod < T : Pod > (& mut self , val : & T) where Self : Sized , { self . write_bytes (bytes_of (val)) } # [doc = " Writes the specified `Pod` slice at the end of the buffer."] fn write_pod_slice < T : Pod > (& mut self , val : & [T]) where Self : Sized , { self . write_bytes (bytes_of_slice (val)) } }
};
}
