// Generated macro for implements_trait (function)
macro_rules! Depcrate_tyimplements_trait {
() => {
// Module: crate::ty
// Provides: {"implements_trait"}
// Dependencies: {}
# [doc = " Checks whether a type implements a trait."] # [doc = " The function returns false in case the type contains an inference variable."] # [doc = ""] # [doc = " See [Common tools for writing lints] for an example how to use this function and other options."] # [doc = ""] # [doc = " [Common tools for writing lints]: https://github.com/rust-lang/rust-clippy/blob/master/book/src/development/common_tools_writing_lints.md#checking-if-a-type-implements-a-specific-trait"] pub fn implements_trait < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , trait_id : DefId , args : & [GenericArg < 'tcx >] ,) -> bool { implements_trait_with_env_from_iter (cx . tcx , cx . typing_env () , ty , trait_id , None , args . iter () . map (| & x | Some (x)) ,) }
};
}
