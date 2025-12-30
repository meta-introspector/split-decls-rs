// Generated macro for ColumnIter (struct)
macro_rules! Depcrate_base_iterColumnIter {
() => {
// Module: crate::base::iter
// Provides: {"ColumnIter"}
// Dependencies: {}
# [derive (Clone , Debug)] # [doc = " An iterator through the columns of a matrix."] pub struct ColumnIter < 'a , T , R : Dim , C : Dim , S : RawStorage < T , R , C > > { mat : & 'a Matrix < T , R , C , S > , range : Range < usize > , }
};
}
