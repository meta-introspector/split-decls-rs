// Generated macro for macro_973 (macro)
macro_rules! Depcrate_base_matrix_viewmacro_973 {
() => {
// Module: crate::base::matrix_view
// Provides: {"macro_973"}
// Dependencies: {}
view_storage_impl ! ("A mutable matrix data storage for mutable matrix view. Only contains an \
                     internal mutable reference to another matrix data storage." ; RawStorageMut as &'a mut S ; SliceStorageMut => ViewStorageMut . get_address_unchecked_mut (* mut T as &'a mut T)) ;
};
}
