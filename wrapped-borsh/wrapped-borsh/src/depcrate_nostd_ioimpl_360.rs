// Generated macro for impl_360 (impl)
macro_rules! Depcrate_nostd_ioimpl_360 {
() => {
// Module: crate::nostd_io
// Provides: {"impl_360"}
// Dependencies: {}
impl Read for & [u8] { # [inline] fn read (& mut self , buf : & mut [u8]) -> Result < usize > { let amt = core :: cmp :: min (buf . len () , self . len ()) ; let (a , b) = self . split_at (amt) ; if amt == 1 { buf [0] = a [0] ; } else { buf [.. amt] . copy_from_slice (a) ; } * self = b ; Ok (amt) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> Result < () > { if buf . len () > self . len () { return Err (Error :: new (ErrorKind :: UnexpectedEof , "failed to fill whole buffer" ,)) ; } let (a , b) = self . split_at (buf . len ()) ; if buf . len () == 1 { buf [0] = a [0] ; } else { buf . copy_from_slice (a) ; } * self = b ; Ok (()) } }
};
}
