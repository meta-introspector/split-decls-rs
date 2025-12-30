// Generated macro for impl_495 (impl)
macro_rules! Depcrate_contextimpl_495 {
() => {
// Module: crate::context
// Provides: {"impl_495"}
// Dependencies: {}
impl < 'a > ContextBase < 'a , & 'a Positioned < Directive > > { # [doc (hidden)] pub fn param_value < T : InputType > (& self , name : & str , default : Option < fn () -> T > ,) -> ServerResult < (Pos , T) > { self . get_param_value (& self . item . node . arguments , name , default) } }
};
}
