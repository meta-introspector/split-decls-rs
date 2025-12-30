// Generated macro for RowIter (struct)
macro_rules! Depcrate_base_iterRowIter {
() => {
// Module: crate::base::iter
// Provides: {"RowIter"}
// Dependencies: {}
# [derive (Clone , Debug)] # [doc = " An iterator through the rows of a matrix."] pub struct RowIter < 'a , T , R : Dim , C : Dim , S : RawStorage < T , R , C > > { mat : & 'a Matrix < T , R , C , S > , curr : usize , }
};
}
