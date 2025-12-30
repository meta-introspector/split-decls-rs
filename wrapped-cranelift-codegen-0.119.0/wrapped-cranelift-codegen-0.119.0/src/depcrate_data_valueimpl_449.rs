// Generated macro for impl_449 (impl)
macro_rules! Depcrate_data_valueimpl_449 {
() => {
// Module: crate::data_value
// Provides: {"impl_449"}
// Dependencies: {}
impl < 'a > Display for DisplayDataValues < 'a > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { if self . 0 . len () == 1 { write ! (f , "{}" , self . 0 [0]) } else { write ! (f , "[") ? ; write_data_value_list (f , & self . 0) ? ; write ! (f , "]") } } }
};
}
