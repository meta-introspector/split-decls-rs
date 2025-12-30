// Generated macro for update_type_with_lifetime (function)
macro_rules! Depcrate_render_apply_argumentsupdate_type_with_lifetime {
() => {
// Module: crate::render::apply_arguments
// Provides: {"update_type_with_lifetime"}
// Dependencies: {}
fn update_type_with_lifetime (ty : & mut Type , ident : Ident) -> Option < Lifetime > { if let Type :: Reference (ty_ref @ TypeReference { lifetime : None , .. }) = ty { let lifetime = Some (syn :: Lifetime { apostrophe : ident . span () , ident , }) ; ty_ref . lifetime . clone_from (& lifetime) ; lifetime } else { None } }
};
}
