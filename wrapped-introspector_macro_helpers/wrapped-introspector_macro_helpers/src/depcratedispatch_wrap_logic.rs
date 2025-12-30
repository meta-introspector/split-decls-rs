// Generated macro for dispatch_wrap_logic (macro)
macro_rules! Depcratedispatch_wrap_logic {
() => {
// Module: crate
// Provides: {"dispatch_wrap_logic"}
// Dependencies: {}
# [macro_export] macro_rules ! dispatch_wrap_logic { ($ item_token_stream : expr , $ args_struct : expr) => { { let item_clone = $ item_token_stream . clone () ; let args = $ args_struct ; if let Ok (mut item_fn) = syn :: parse2 ::< syn :: ItemFn > (item_clone . clone ()) { crate :: wrap_function (& args , & mut item_fn) } else if let Ok (mut item_struct) = syn :: parse2 ::< syn :: ItemStruct > (item_clone . clone ()) { crate :: wrap_struct (& args , & mut item_struct) } else if let Ok (mut item_enum) = syn :: parse2 ::< syn :: ItemEnum > (item_clone . clone ()) { crate :: wrap_enum (& args , & mut item_enum) } else if let Ok (mut item_trait) = syn :: parse2 ::< syn :: ItemTrait > (item_clone . clone ()) { crate :: wrap_trait (& args , & mut item_trait) } else { $ item_token_stream } } } ; }
};
}
