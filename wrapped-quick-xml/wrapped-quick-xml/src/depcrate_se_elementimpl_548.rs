// Generated macro for impl_548 (impl)
macro_rules! Depcrate_se_elementimpl_548 {
() => {
// Module: crate::se::element
// Provides: {"impl_548"}
// Dependencies: {}
impl < 'w , 'k , W : Write > Map < 'w , 'k , W > { fn make_key < T > (& mut self , key : & T) -> Result < String , SeError > where T : ? Sized + Serialize , { key . serialize (QNameSerializer { writer : String :: new () , }) } }
};
}
