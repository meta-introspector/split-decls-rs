// Generated macro for Context (struct)
macro_rules! Depcrate_pin_project_deriveContext {
() => {
// Module: crate::pin_project::derive
// Provides: {"Context"}
// Dependencies: {}
struct Context < 'a > { # [doc = " The original type."] orig : OriginalType < 'a > , # [doc = " The projected types."] proj : ProjectedType , # [doc = " Types of the pinned fields."] pinned_fields : Vec < & 'a Type > , # [doc = " Kind of the original type: struct or enum"] kind : TypeKind , # [doc = " `PinnedDrop` argument."] pinned_drop : Option < Span > , # [doc = " `UnsafeUnpin` or `!Unpin` argument."] unpin_impl : UnpinImpl , # [doc = " `project` argument."] project : bool , # [doc = " `project_ref` argument."] project_ref : bool , # [doc = " `project_replace [= <ident>]` argument."] project_replace : ProjReplace , }
};
}
