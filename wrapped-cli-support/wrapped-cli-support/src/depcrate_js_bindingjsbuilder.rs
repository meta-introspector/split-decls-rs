// Generated macro for JsBuilder (struct)
macro_rules! Depcrate_js_bindingJsBuilder {
() => {
// Module: crate::js::binding
// Provides: {"JsBuilder"}
// Dependencies: {}
# [doc = " Helper struct used to create JS to process all instructions in an adapter"] # [doc = " function."] pub struct JsBuilder < 'a , 'b > { # [doc = " General context for building JS, used to manage intrinsic names, exposed"] # [doc = " JS functions, etc."] cx : & 'a mut Context < 'b > , # [doc = " A debug name for the function being generated, used for error messages"] debug_name : & 'a str , # [doc = " The \"prelude\" of the function, or largely just the JS function we've"] # [doc = " built so far."] prelude : String , # [doc = " Code which should go before the `try {` in a try-finally block."] pre_try : String , # [doc = " JS code to execute in a `finally` block in case any exceptions happen."] finally : String , # [doc = " An index used to manage allocation of temporary indices, used to name"] # [doc = " variables to ensure nothing clashes with anything else."] tmp : usize , # [doc = " Names or expressions representing the arguments to the adapter. This is"] # [doc = " use to translate the `arg.get` instruction."] args : Vec < String > , # [doc = " The Wasm interface types \"stack\". The expressions pushed onto this stack"] # [doc = " are intended to be *pure*, and if they're not, they should be pushed"] # [doc = " into the `prelude`, assigned to a variable, and the variable should be"] # [doc = " pushed to the stack. We're not super principled about this though, so"] # [doc = " improvements will likely happen here over time."] stack : Vec < String > , }
};
}
