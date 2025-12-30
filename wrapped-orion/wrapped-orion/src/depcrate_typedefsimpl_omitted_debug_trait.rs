// Generated macro for impl_omitted_debug_trait (macro)
macro_rules! Depcrate_typedefsimpl_omitted_debug_trait {
() => {
// Module: crate::typedefs
// Provides: {"impl_omitted_debug_trait"}
// Dependencies: {}
# [doc = " Macro that implements the `Debug` trait on a object called `$name`."] # [doc = " This `Debug` will omit any fields of object `$name` to avoid them being"] # [doc = " written to logs."] macro_rules ! impl_omitted_debug_trait (($ name : ident) => (impl core :: fmt :: Debug for $ name { fn fmt (& self , f : & mut core :: fmt :: Formatter <'_ >) -> core :: fmt :: Result { write ! (f , "{} {{***OMITTED***}}" , stringify ! ($ name)) } })) ;
};
}
