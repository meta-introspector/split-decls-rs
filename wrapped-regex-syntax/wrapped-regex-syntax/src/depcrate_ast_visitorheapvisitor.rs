// Generated macro for HeapVisitor (struct)
macro_rules! Depcrate_ast_visitorHeapVisitor {
() => {
// Module: crate::ast::visitor
// Provides: {"HeapVisitor"}
// Dependencies: {}
# [doc = " HeapVisitor visits every item in an `Ast` recursively using constant stack"] # [doc = " size and a heap size proportional to the size of the `Ast`."] struct HeapVisitor < 'a > { # [doc = " A stack of `Ast` nodes. This is roughly analogous to the call stack"] # [doc = " used in a typical recursive visitor."] stack : Vec < (& 'a Ast , Frame < 'a >) > , # [doc = " Similar to the `Ast` stack above, but is used only for character"] # [doc = " classes. In particular, character classes embed their own mini"] # [doc = " recursive syntax."] stack_class : Vec < (ClassInduct < 'a > , ClassFrame < 'a >) > , }
};
}
