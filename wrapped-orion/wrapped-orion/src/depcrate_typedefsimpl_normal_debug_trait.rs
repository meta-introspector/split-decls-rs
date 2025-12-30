// Generated macro for impl_normal_debug_trait (macro)
macro_rules! Depcrate_typedefsimpl_normal_debug_trait {
() => {
// Module: crate::typedefs
// Provides: {"impl_normal_debug_trait"}
// Dependencies: {}
# [doc = " Macro that implements the `Debug` trait on a object called `$name`."] macro_rules ! impl_normal_debug_trait (($ name : ident) => (impl core :: fmt :: Debug for $ name { fn fmt (& self , f : & mut core :: fmt :: Formatter <'_ >) -> core :: fmt :: Result { write ! (f , "{} {:?}" , stringify ! ($ name) , & self . value [..]) } })) ;
};
}
