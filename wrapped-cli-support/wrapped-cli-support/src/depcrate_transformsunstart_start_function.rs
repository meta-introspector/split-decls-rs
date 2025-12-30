// Generated macro for unstart_start_function (function)
macro_rules! Depcrate_transformsunstart_start_function {
() => {
// Module: crate::transforms
// Provides: {"unstart_start_function"}
// Dependencies: {}
# [doc = " If a start function is present, it removes it from the `start` section"] # [doc = " of the Wasm module and then moves it to an exported function, named"] # [doc = " `__wbindgen_start`."] pub (crate) fn unstart_start_function (module : & mut walrus :: Module) -> bool { let start = match module . start . take () { Some (id) => id , None => return false , } ; module . exports . add ("__wbindgen_start" , start) ; true }
};
}
