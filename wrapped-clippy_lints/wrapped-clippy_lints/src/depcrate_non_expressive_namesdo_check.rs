// Generated macro for do_check (function)
macro_rules! Depcrate_non_expressive_namesdo_check {
() => {
// Module: crate::non_expressive_names
// Provides: {"do_check"}
// Dependencies: {}
fn do_check (lint : & NonExpressiveNames , cx : & EarlyContext < '_ > , attrs : & [Attribute] , decl : & FnDecl , blk : & Block) { if ! attrs . iter () . any (| attr | attr . has_name (sym :: test)) { let mut visitor = SimilarNamesLocalVisitor { names : Vec :: new () , cx , threshold : lint . single_char_binding_names_threshold , single_char_names : vec ! [vec ! []] , } ; for arg in & decl . inputs { SimilarNamesNameVisitor (& mut visitor) . visit_pat (& arg . pat) ; } walk_block (& mut visitor , blk) ; visitor . check_single_char_names () ; } }
};
}
