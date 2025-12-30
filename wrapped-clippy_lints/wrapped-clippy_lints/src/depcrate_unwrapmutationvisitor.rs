// Generated macro for MutationVisitor (struct)
macro_rules! Depcrate_unwrapMutationVisitor {
() => {
// Module: crate::unwrap
// Provides: {"MutationVisitor"}
// Dependencies: {}
# [doc = " A HIR visitor delegate that checks if a local variable of type `Option` or `Result` is mutated,"] # [doc = " *except* for if `.as_mut()` is called."] # [doc = " The reason for why we allow that one specifically is that `.as_mut()` cannot change"] # [doc = " the variant, and that is important because this lint relies on the fact that"] # [doc = " `is_some` + `unwrap` is equivalent to `if let Some(..) = ..`, which it would not be if"] # [doc = " the option is changed to None between `is_some` and `unwrap`, ditto for `Result`."] # [doc = " (And also `.as_mut()` is a somewhat common method that is still worth linting on.)"] struct MutationVisitor < 'tcx , 'lcl > { is_mutated : bool , local : & 'lcl Local , tcx : TyCtxt < 'tcx > , }
};
}
