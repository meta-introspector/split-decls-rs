// Generated macro for macro_972 (macro)
macro_rules! Depcrate_base_matrix_viewmacro_972 {
() => {
// Module: crate::base::matrix_view
// Provides: {"macro_972"}
// Dependencies: {}
view_storage_impl ! ("A matrix data storage for a matrix view. Only contains an internal reference \
                     to another matrix data storage." ; RawStorage as &'a S ; SliceStorage => ViewStorage . get_address_unchecked (* const T as &'a T)) ;
};
}
