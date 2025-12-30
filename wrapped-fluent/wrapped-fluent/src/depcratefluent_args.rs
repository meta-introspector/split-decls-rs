// Generated macro for fluent_args (macro)
macro_rules! Depcratefluent_args {
() => {
// Module: crate
// Provides: {"fluent_args"}
// Dependencies: {}
# [doc = " A helper macro to simplify creation of `FluentArgs`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent::fluent_args;"] # [doc = ""] # [doc = " let mut args = fluent_args!["] # [doc = "     \"name\" => \"John\","] # [doc = "     \"emailCount\" => 5,"] # [doc = " ];"] # [doc = ""] # [doc = " ```"] # [macro_export] macro_rules ! fluent_args { ($ ($ key : expr => $ value : expr) ,* $ (,) ?) => { { let mut args : $ crate :: FluentArgs = $ crate :: FluentArgs :: new () ; $ (args . set ($ key , $ value) ;) * args } } ; }
};
}
