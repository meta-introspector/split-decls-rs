// Generated macro for assert_path_len (macro)
macro_rules! Depcrate_rt_pathassert_path_len {
() => {
// Module: crate::rt::path
// Provides: {"assert_path_len"}
// Dependencies: {}
macro_rules ! assert_path_len { ($ branches : expr) => { { assert ! ($ branches . len () < $ branches . capacity () || std :: thread :: panicking () , "Model exceeded maximum number of branches. This is often caused \
             by an algorithm requiring the processor to make progress, e.g. \
             spin locks." ,) ; } } ; }
};
}
