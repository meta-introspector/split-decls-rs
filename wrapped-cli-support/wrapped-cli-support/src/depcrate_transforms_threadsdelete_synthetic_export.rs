// Generated macro for delete_synthetic_export (function)
macro_rules! Depcrate_transforms_threadsdelete_synthetic_export {
() => {
// Module: crate::transforms::threads
// Provides: {"delete_synthetic_export"}
// Dependencies: {}
fn delete_synthetic_export (module : & mut Module , name : & str) -> Result < ExportItem , Error > { let item = module . exports . iter () . find (| e | e . name == name) . ok_or_else (| | anyhow ! ("failed to find `{name}`")) ? ; let ret = item . item ; let id = item . id () ; module . exports . delete (id) ; Ok (ret) }
};
}
