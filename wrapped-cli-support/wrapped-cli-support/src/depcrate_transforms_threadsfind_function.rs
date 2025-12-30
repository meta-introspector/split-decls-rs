// Generated macro for find_function (function)
macro_rules! Depcrate_transforms_threadsfind_function {
() => {
// Module: crate::transforms::threads
// Provides: {"find_function"}
// Dependencies: {}
fn find_function (module : & Module , name : & str) -> Result < FunctionId , Error > { let e = module . exports . iter () . find (| e | e . name == name) . ok_or_else (| | anyhow ! ("failed to find `{name}`")) ? ; match e . item { walrus :: ExportItem :: Function (f) => Ok (f) , _ => bail ! ("`{name}` wasn't a function") , } }
};
}
