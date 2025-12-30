// Generated macro for ColumnProducerMut (struct)
macro_rules! Depcrate_base_par_iterColumnProducerMut {
() => {
// Module: crate::base::par_iter
// Provides: {"ColumnProducerMut"}
// Dependencies: {}
# [doc = " See `ColumnProducer`. A private wrapper newtype that keeps the Producer"] # [doc = " implementation private"] struct ColumnProducerMut < 'a , T , R : Dim , C : Dim , S : RawStorageMut < T , R , C > > (ColumnIterMut < 'a , T , R , C , S > ,) ;
};
}
