// Generated macro for impl_64 (impl)
macro_rules! Depcrate_bit_setimpl_64 {
() => {
// Module: crate::bit_set
// Provides: {"impl_64"}
// Dependencies: {}
impl < R : Idx , C : Idx > fmt :: Debug for BitMatrix < R , C > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [doc = " Forces its contents to print in regular mode instead of alternate mode."] struct OneLinePrinter < T > (T) ; impl < T : fmt :: Debug > fmt :: Debug for OneLinePrinter < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "{:?}" , self . 0) } } write ! (fmt , "BitMatrix({}x{}) " , self . num_rows , self . num_columns) ? ; let items = self . rows () . flat_map (| r | self . iter (r) . map (move | c | (r , c))) ; fmt . debug_set () . entries (items . map (OneLinePrinter)) . finish () } }
};
}
