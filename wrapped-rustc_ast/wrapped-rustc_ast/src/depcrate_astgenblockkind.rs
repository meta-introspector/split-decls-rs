// Generated macro for GenBlockKind (enum)
macro_rules! Depcrate_astGenBlockKind {
() => {
// Module: crate::ast
// Provides: {"GenBlockKind"}
// Dependencies: {}
# [doc = " Used to differentiate between `async {}` blocks and `gen {}` blocks."] # [derive (Clone , Encodable , Decodable , Debug , PartialEq , Eq , Walkable)] pub enum GenBlockKind { Async , Gen , AsyncGen , }
};
}
