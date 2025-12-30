// Generated macro for write_to_spare_capacity_of_vec (function)
macro_rules! Depcrate_memwrite_to_spare_capacity_of_vec {
() => {
// Module: crate::mem
// Provides: {"write_to_spare_capacity_of_vec"}
// Dependencies: {}
# [doc = " Allows `writer` to write data into the spare capacity of the `output` vector."] # [doc = " This will not reallocate the vector provided or attempt to grow it, so space"] # [doc = " for the `output` must be reserved by the caller before calling this"] # [doc = " function."] # [doc = ""] # [doc = " `writer` needs to return the number of bytes written (and can also return"] # [doc = " another arbitrary return value)."] # [doc = ""] # [doc = " # Safety:"] # [doc = ""] # [doc = " The length returned by the `writer` must be equal to actual number of bytes written"] # [doc = " to the uninitialized slice passed in and initialized."] unsafe fn write_to_spare_capacity_of_vec < T > (output : & mut Vec < u8 > , writer : impl FnOnce (& mut [MaybeUninit < u8 >]) -> (usize , T) ,) -> T { let cap = output . capacity () ; let len = output . len () ; let (bytes_written , ret) = writer (output . spare_capacity_mut ()) ; output . set_len (cap . min (len + bytes_written)) ; ret }
};
}
