// Generated macro for ColumnProducer (struct)
macro_rules! Depcrate_base_par_iterColumnProducer {
() => {
// Module: crate::base::par_iter
// Provides: {"ColumnProducer"}
// Dependencies: {}
# [doc = " A private helper newtype that wraps the `ColumnIter` and implements"] # [doc = " the rayon `Producer` trait. It's just here so we don't have to make the"] # [doc = " rayon trait part of the public interface of the `ColumnIter`."] struct ColumnProducer < 'a , T , R : Dim , C : Dim , S : RawStorage < T , R , C > > (ColumnIter < 'a , T , R , C , S >) ;
};
}
