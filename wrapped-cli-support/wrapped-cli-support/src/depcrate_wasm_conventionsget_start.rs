// Generated macro for get_start (function)
macro_rules! Depcrate_wasm_conventionsget_start {
() => {
// Module: crate::wasm_conventions
// Provides: {"get_start"}
// Dependencies: {}
pub fn get_start (module : & mut Module) -> Result < FunctionId , Option < FunctionId > > { match module . start { Some (start) => match module . funcs . get_mut (start) . kind { FunctionKind :: Import (_) => Err (Some (start)) , FunctionKind :: Local (_) => Ok (start) , FunctionKind :: Uninitialized (_) => unimplemented ! () , } , None => Err (None) , } }
};
}
