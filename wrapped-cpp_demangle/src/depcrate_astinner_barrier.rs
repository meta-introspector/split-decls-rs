// Generated macro for inner_barrier (macro)
macro_rules! Depcrate_astinner_barrier {
() => {
// Module: crate::ast
// Provides: {"inner_barrier"}
// Dependencies: {}
# [doc = " The inner stack allows passing AST nodes down deeper into the tree so that"] # [doc = " nodes that logically precede something (e.g. PointerRef) can show up after"] # [doc = " that thing in the demangled output. What's on the stack may not always be"] # [doc = " intended for the first node that looks at the stack to grab, though."] # [doc = ""] # [doc = " Consider a function with template arguments and parameters, f<T>(a)."] # [doc = " The function parameters logically precede the template arguments in the AST,"] # [doc = " but they must be reversed in the output. The parameters end up on the inner"] # [doc = " stack before processing the template argument nodes. If we're not careful,"] # [doc = " a node inside the template arguments might pick the function parameters"] # [doc = " off of the inner stack!"] # [doc = ""] # [doc = " To solve this, certain nodes act as \"inner barriers\". By using this macro,"] # [doc = " they set the existing inner stack aside and replace it with an empty stack"] # [doc = " while visiting their children. This allows these barrier nodes to have"] # [doc = " completely self-contained children."] macro_rules ! inner_barrier { ($ ctx : ident) => { let mut _ctx = AutoDemangleContextInnerBarrier :: new ($ ctx) ; let $ ctx = & mut _ctx ; } ; }
};
}
