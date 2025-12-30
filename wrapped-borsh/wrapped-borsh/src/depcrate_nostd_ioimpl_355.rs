// Generated macro for impl_355 (impl)
macro_rules! Depcrate_nostd_ioimpl_355 {
() => {
// Module: crate::nostd_io
// Provides: {"impl_355"}
// Dependencies: {}
# [doc = " Write is implemented for `&mut [u8]` by copying into the slice, overwriting"] # [doc = " its data."] # [doc = ""] # [doc = " Note that writing updates the slice to point to the yet unwritten part."] # [doc = " The slice will be empty when it has been completely overwritten."] impl Write for & mut [u8] { # [inline] fn write (& mut self , data : & [u8]) -> Result < usize > { let amt = core :: cmp :: min (data . len () , self . len ()) ; let (a , b) = core :: mem :: replace (self , & mut []) . split_at_mut (amt) ; a . copy_from_slice (& data [.. amt]) ; * self = b ; Ok (amt) } # [inline] fn write_all (& mut self , data : & [u8]) -> Result < () > { if self . write (data) ? == data . len () { Ok (()) } else { Err (Error :: new (ErrorKind :: WriteZero , "failed to write whole buffer" ,)) } } # [inline] fn flush (& mut self) -> Result < () > { Ok (()) } }
};
}
