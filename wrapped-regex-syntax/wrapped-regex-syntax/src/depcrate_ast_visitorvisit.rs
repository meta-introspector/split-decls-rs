// Generated macro for visit (function)
macro_rules! Depcrate_ast_visitorvisit {
() => {
// Module: crate::ast::visitor
// Provides: {"visit"}
// Dependencies: {}
# [doc = " Executes an implementation of `Visitor` in constant stack space."] # [doc = ""] # [doc = " This function will visit every node in the given `Ast` while calling the"] # [doc = " appropriate methods provided by the [`Visitor`] trait."] # [doc = ""] # [doc = " The primary use case for this method is when one wants to perform case"] # [doc = " analysis over an `Ast` without using a stack size proportional to the depth"] # [doc = " of the `Ast`. Namely, this method will instead use constant stack size, but"] # [doc = " will use heap space proportional to the size of the `Ast`. This may be"] # [doc = " desirable in cases where the size of `Ast` is proportional to end user"] # [doc = " input."] # [doc = ""] # [doc = " If the visitor returns an error at any point, then visiting is stopped and"] # [doc = " the error is returned."] pub fn visit < V : Visitor > (ast : & Ast , visitor : V) -> Result < V :: Output , V :: Err > { HeapVisitor :: new () . visit (ast , visitor) }
};
}
