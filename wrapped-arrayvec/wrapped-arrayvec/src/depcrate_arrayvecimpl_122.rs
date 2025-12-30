// Generated macro for impl_122 (impl)
macro_rules! Depcrate_arrayvecimpl_122 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_122"}
// Dependencies: {}
# [cfg (feature = "std")] # [doc = " `Write` appends written data to the end of the vector."] # [doc = ""] # [doc = " Requires `features=\"std\"`."] impl < const CAP : usize > io :: Write for ArrayVec < u8 , CAP > { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { let len = cmp :: min (self . remaining_capacity () , data . len ()) ; let _result = self . try_extend_from_slice (& data [.. len]) ; debug_assert ! (_result . is_ok ()) ; Ok (len) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
