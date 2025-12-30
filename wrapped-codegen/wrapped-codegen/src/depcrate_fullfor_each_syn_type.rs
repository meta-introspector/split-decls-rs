// Generated macro for for_each_syn_type (function)
macro_rules! Depcrate_fullfor_each_syn_type {
() => {
// Module: crate::full
// Provides: {"for_each_syn_type"}
// Dependencies: {}
fn for_each_syn_type (ty : & Type , f : & mut dyn FnMut (& str)) { match ty { Type :: Syn (ty) => f (ty) , Type :: Std (_) | Type :: Ext (_) | Type :: Token (_) | Type :: Group (_) => { } Type :: Punctuated (punctuated) => for_each_syn_type (& punctuated . element , f) , Type :: Option (ty) | Type :: Box (ty) | Type :: Vec (ty) => for_each_syn_type (ty , f) , Type :: Tuple (elements) => { for ty in elements { for_each_syn_type (ty , f) ; } } } }
};
}
