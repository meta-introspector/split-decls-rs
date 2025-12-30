// Generated macro for impl_574 (impl)
macro_rules! Depcrate_se_simple_typeimpl_574 {
() => {
// Module: crate::se::simple_type
// Provides: {"impl_574"}
// Dependencies: {}
impl < W : Write > SimpleTypeSerializer < W > { # [inline] fn write_str (& mut self , value : & str) -> Result < () , SeError > { Ok (self . writer . write_str (value) ?) } # [inline] fn write_fmt (& mut self , args : fmt :: Arguments < '_ >) -> Result < () , SeError > { Ok (self . writer . write_fmt (args) ?) } }
};
}
