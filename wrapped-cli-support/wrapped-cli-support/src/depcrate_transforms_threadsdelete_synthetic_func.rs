// Generated macro for delete_synthetic_func (function)
macro_rules! Depcrate_transforms_threadsdelete_synthetic_func {
() => {
// Module: crate::transforms::threads
// Provides: {"delete_synthetic_func"}
// Dependencies: {}
fn delete_synthetic_func (module : & mut Module , name : & str) -> Result < FunctionId , Error > { match delete_synthetic_export (module , name) ? { walrus :: ExportItem :: Function (f) => Ok (f) , _ => bail ! ("`{name}` must be a function") , } }
};
}
