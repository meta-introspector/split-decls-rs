// Generated macro for Dict (struct)
macro_rules! Depcrate_arg_array_implDict {
() => {
// Module: crate::arg::array_impl
// Provides: {"Dict"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [doc = " Append a D-Bus dict type (i e, an array of dict entries)."] # [doc = ""] # [doc = " See the argument guide and module level documentation for details and alternatives."] pub struct Dict < 'a , K : DictKey , V : Arg , I > (I , PhantomData < (& 'a Message , * const K , * const V) >) ;
};
}
