// Generated macro for projection_has_drop_glue (function)
macro_rules! Depcrate_dropprojection_has_drop_glue {
() => {
// Module: crate::drop
// Provides: {"projection_has_drop_glue"}
// Dependencies: {}
fn projection_has_drop_glue (db : & dyn HirDatabase , env : Arc < TraitEnvironment > , projection : ProjectionTy , ty : Ty ,) -> DropGlue { let normalized = db . normalize_projection (projection , env . clone ()) ; match normalized . kind (Interner) { TyKind :: Alias (AliasTy :: Projection (_)) | TyKind :: AssociatedType (..) => { if is_copy (db , ty , env) { DropGlue :: None } else { DropGlue :: DependOnParams } } _ => db . has_drop_glue (normalized , env) , } }
};
}
