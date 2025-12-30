// Generated macro for splat (function)
macro_rules! Depcrate_codegensplat {
() => {
// Module: crate::codegen
// Provides: {"splat"}
// Dependencies: {}
# [doc = " Splats an argument with the given name and ABI type into 4 arguments, one"] # [doc = " for each primitive that the ABI type splits into."] # [doc = ""] # [doc = " Returns an `(args, names)` pair, where `args` is the list of arguments to"] # [doc = " be inserted into the function signature, and `names` is a list of the names"] # [doc = " of those arguments."] fn splat (wasm_bindgen : & syn :: Path , name : & Ident , abi : & TokenStream ,) -> (Vec < TokenStream > , Vec < Ident >) { let mut args = Vec :: new () ; let mut names = Vec :: new () ; for n in 1_u32 ..= 4 { let arg_name = format_ident ! ("{}_{}" , name , n) ; let prim_name = format_ident ! ("Prim{}" , n) ; args . push (quote ! { # arg_name : <# abi as # wasm_bindgen :: convert :: WasmAbi >::# prim_name }) ; names . push (arg_name) ; } (args , names) }
};
}
