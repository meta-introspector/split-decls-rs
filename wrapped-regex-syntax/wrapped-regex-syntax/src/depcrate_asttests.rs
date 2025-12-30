// Generated macro for tests (module)
macro_rules! Depcrate_asttests {
() => {
// Module: crate::ast
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [cfg (any (unix , windows))] fn no_stack_overflow_on_drop () { use std :: thread ; let run = | | { let span = | | Span :: splat (Position :: new (0 , 0 , 0)) ; let mut ast = Ast :: empty (span ()) ; for i in 0 .. 200 { ast = Ast :: group (Group { span : span () , kind : GroupKind :: CaptureIndex (i) , ast : Box :: new (ast) , }) ; } assert ! (! ast . is_empty ()) ; } ; thread :: Builder :: new () . stack_size (16 << 10) . spawn (run) . unwrap () . join () . unwrap () ; } # [test] fn ast_size () { let max = 2 * core :: mem :: size_of :: < usize > () ; let size = core :: mem :: size_of :: < Ast > () ; assert ! (size <= max , "Ast size of {size} bytes is bigger than suggested max {max}" ,) ; } }
};
}
