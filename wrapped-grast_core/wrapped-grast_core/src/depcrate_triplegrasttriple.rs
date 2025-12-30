// Generated macro for GrastTriple (struct)
macro_rules! Depcrate_tripleGrastTriple {
() => {
// Module: crate::triple
// Provides: {"GrastTriple"}
// Dependencies: {}
# [doc = " Core grast representation: flat triple format"] # [doc = " Format: subject predicate object"] # [doc = " Example: node_0 :type :FunctionDecl"] # [doc = "          node_0 :name \"main\""] # [doc = "          node_0 :child node_1"] # [derive (Debug , Clone)] # [decl (struct , name = "GrastTriple" , vis = "pub" , hash = "5fa5b99d")] pub struct GrastTriple { pub subject : String , pub predicate : String , pub object : String , }
};
}
