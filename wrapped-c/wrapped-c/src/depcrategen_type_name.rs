// Generated macro for gen_type_name (function)
macro_rules! Depcrategen_type_name {
() => {
// Module: crate
// Provides: {"gen_type_name"}
// Dependencies: {}
# [doc = " Generate the type part of a c identifier, missing the namespace and the `_t` suffix."] # [doc = " Additionally return a `CTypeNameInfo` that describes what sort of name has been produced."] pub fn gen_type_name (resolve : & Resolve , ty : TypeId) -> (CTypeNameInfo < '_ > , String) { let mut encoded = String :: new () ; push_ty_name (resolve , & Type :: Id (ty) , & mut encoded) ; let info = if let Some (name) = & resolve . types [ty] . name { CTypeNameInfo :: Named { name : name . as_ref () , } } else { CTypeNameInfo :: Anonymous { is_prim : is_prim_type_id (resolve , ty) , } } ; (info , encoded) }
};
}
