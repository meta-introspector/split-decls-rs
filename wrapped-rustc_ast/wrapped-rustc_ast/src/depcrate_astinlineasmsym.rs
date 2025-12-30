// Generated macro for InlineAsmSym (struct)
macro_rules! Depcrate_astInlineAsmSym {
() => {
// Module: crate::ast
// Provides: {"InlineAsmSym"}
// Dependencies: {}
# [doc = " Inline assembly symbol operands get their own AST node that is somewhat"] # [doc = " similar to `AnonConst`."] # [doc = ""] # [doc = " The main difference is that we specifically don't assign it `DefId` in"] # [doc = " `DefCollector`. Instead this is deferred until AST lowering where we"] # [doc = " lower it to an `AnonConst` (for functions) or a `Path` (for statics)"] # [doc = " depending on what the path resolves to."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct InlineAsmSym { pub id : NodeId , pub qself : Option < Box < QSelf > > , pub path : Path , }
};
}
