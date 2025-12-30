// Generated macro for Ty (enum)
macro_rules! Depcrate_deriving_generic_tyTy {
() => {
// Module: crate::deriving::generic::ty
// Provides: {"Ty"}
// Dependencies: {}
# [doc = " A type. Supports pointers, Self, literals, unit or an arbitrary AST path."] # [derive (Clone)] pub (crate) enum Ty { Self_ , # [doc = " A reference."] Ref (Box < Ty > , ast :: Mutability) , # [doc = " `mod::mod::Type<[lifetime], [Params...]>`, including a plain type"] # [doc = " parameter, and things like `i32`"] Path (Path) , # [doc = " For () return types."] Unit , # [doc = " An arbitrary type."] AstTy (Box < ast :: Ty >) , }
};
}
