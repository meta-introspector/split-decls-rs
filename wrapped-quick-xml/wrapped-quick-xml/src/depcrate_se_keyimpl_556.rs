// Generated macro for impl_556 (impl)
macro_rules! Depcrate_se_keyimpl_556 {
() => {
// Module: crate::se::key
// Provides: {"impl_556"}
// Dependencies: {}
impl < W : Write > QNameSerializer < W > { # [inline] fn write_str (& mut self , value : & str) -> Result < () , SeError > { Ok (self . writer . write_str (value) ?) } # [inline] fn write_fmt (& mut self , args : fmt :: Arguments < '_ >) -> Result < () , SeError > { Ok (self . writer . write_fmt (args) ?) } }
};
}
