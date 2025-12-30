// Generated macro for impl_405 (impl)
macro_rules! Depcrate_renderimpl_405 {
() => {
// Module: crate::render
// Provides: {"impl_405"}
// Dependencies: {}
impl Evaluable for TemplateElement { fn eval < 'reg : 'rc , 'rc > (& 'rc self , registry : & 'reg Registry < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > ,) -> Result < () , RenderError > { match * self { DecoratorExpression (ref dt) | DecoratorBlock (ref dt) => { let di = Decorator :: try_from_template (dt , registry , ctx , rc) ? ; match registry . get_decorator (di . name ()) { Some (d) => d . call (& di , registry , ctx , rc) , None => Err (RenderErrorReason :: DecoratorNotFound (di . name () . to_owned ()) . into ()) , } } _ => Ok (()) , } } }
};
}
