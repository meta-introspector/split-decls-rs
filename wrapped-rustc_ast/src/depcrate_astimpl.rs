// Generated macro for Impl (struct)
macro_rules! Depcrate_astImpl {
() => {
// Module: crate::ast
// Provides: {"Impl"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug)] pub struct Impl { pub generics : Generics , pub of_trait : Option < Box < TraitImplHeader > > , pub self_ty : Box < Ty > , pub items : ThinVec < Box < AssocItem > > , }
};
}
