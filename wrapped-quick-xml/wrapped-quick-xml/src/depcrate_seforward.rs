// Generated macro for forward (macro)
macro_rules! Depcrate_seforward {
() => {
// Module: crate::se
// Provides: {"forward"}
// Dependencies: {}
# [doc = " Implements serialization method by forwarding it to the serializer created by"] # [doc = " the helper method [`Serializer::ser`]."] macro_rules ! forward { ($ name : ident ($ ty : ty)) => { fn $ name (self , value : $ ty) -> Result < Self :: Ok , Self :: Error > { self . ser (& concat ! ("`" , stringify ! ($ ty) , "`")) ?.$ name (value) } } ; }
};
}
