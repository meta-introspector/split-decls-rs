// Generated macro for BodyVisitor (struct)
macro_rules! Depcrate_macro_metavars_in_unsafeBodyVisitor {
() => {
// Module: crate::macro_metavars_in_unsafe
// Provides: {"BodyVisitor"}
// Dependencies: {}
struct BodyVisitor < 'a , 'tcx > { # [doc = " Stack of unsafe blocks -- the top item always represents the last seen unsafe block from"] # [doc = " within a relevant macro."] macro_unsafe_blocks : Vec < HirId > , # [doc = " When this is >0, it means that the node currently being visited is \"within\" a"] # [doc = " macro definition."] # [doc = " This is used to detect if an expression represents a metavariable."] # [doc = ""] # [doc = " For example, the following pre-expansion code that we want to lint"] # [doc = " ```ignore"] # [doc = " macro_rules! m { ($e:expr) => { unsafe { $e; } } }"] # [doc = " m!(1);"] # [doc = " ```"] # [doc = " would look like this post-expansion code:"] # [doc = " ```ignore"] # [doc = " unsafe { /* macro */"] # [doc = "     1 /* root */; /* macro */"] # [doc = " }"] # [doc = " ```"] # [doc = " Visiting the block and the statement will increment the `expn_depth` so that it is >0,"] # [doc = " and visiting the expression with a root context while `expn_depth > 0` tells us"] # [doc = " that it must be a metavariable."] expn_depth : u32 , cx : & 'a LateContext < 'tcx > , lint : & 'a mut ExprMetavarsInUnsafe , }
};
}
