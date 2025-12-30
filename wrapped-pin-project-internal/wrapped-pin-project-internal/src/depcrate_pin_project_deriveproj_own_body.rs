// Generated macro for proj_own_body (function)
macro_rules! Depcrate_pin_project_deriveproj_own_body {
() => {
// Module: crate::pin_project::derive
// Provides: {"proj_own_body"}
// Dependencies: {}
# [doc = " Generates the processing that `project_replace` does for the struct or each variant."] # [doc = ""] # [doc = " Note: `pinned_fields` must be in declaration order."] fn proj_own_body (cx : & Context < '_ > , variant_ident : Option < & Ident > , proj_move : Option < & Group > , pinned_fields : & [Ident] ,) -> TokenStream { let ident = & cx . proj . own_ident ; let proj_own = match variant_ident { Some (variant_ident) => quote ! (# ident ::# variant_ident) , None => quote ! (# ident) , } ; let pinned_fields = pinned_fields . iter () . rev () ; quote ! { let __result = # proj_own # proj_move ; { # (let __guard = _pin_project :: __private :: UnsafeDropInPlaceGuard :: new (# pinned_fields) ;) * } __result } }
};
}
