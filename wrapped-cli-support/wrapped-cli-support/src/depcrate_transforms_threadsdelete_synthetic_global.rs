// Generated macro for delete_synthetic_global (function)
macro_rules! Depcrate_transforms_threadsdelete_synthetic_global {
() => {
// Module: crate::transforms::threads
// Provides: {"delete_synthetic_global"}
// Dependencies: {}
fn delete_synthetic_global (module : & mut Module , name : & str) -> Result < u32 , Error > { let id = match delete_synthetic_export (module , name) ? { walrus :: ExportItem :: Global (g) => g , _ => bail ! ("`{name}` must be a global") , } ; let g = match & module . globals . get (id) . kind { walrus :: GlobalKind :: Local (g) => g , walrus :: GlobalKind :: Import (_) => bail ! ("`{name}` must not be an imported global") , } ; match g { ConstExpr :: Value (Value :: I32 (v)) => Ok (* v as u32) , _ => bail ! ("`{name}` was not an `i32` constant") , } }
};
}
