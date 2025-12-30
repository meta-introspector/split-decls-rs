// Generated macro for DeferredTaskReturn (enum)
macro_rules! DepcrateDeferredTaskReturn {
() => {
// Module: crate
// Provides: {"DeferredTaskReturn"}
// Dependencies: {}
# [derive (Clone , Debug)] enum DeferredTaskReturn { None , Generating { prev_src : String , return_param : String , } , Emitted { params : Vec < (WasmType , String) > , body : String , return_param : String , } , }
};
}
