// Generated macro for read_explicit_enum_value (function)
macro_rules! Depcrate_tyread_explicit_enum_value {
() => {
// Module: crate::ty
// Provides: {"read_explicit_enum_value"}
// Dependencies: {}
# [doc = " Attempts to read the given constant as though it were an enum value."] pub fn read_explicit_enum_value (tcx : TyCtxt < '_ > , id : DefId) -> Option < EnumValue > { if let Ok (ConstValue :: Scalar (Scalar :: Int (value))) = tcx . const_eval_poly (id) { match tcx . type_of (id) . instantiate_identity () . kind () { ty :: Int (_) => Some (EnumValue :: Signed (value . to_int (value . size ()))) , ty :: Uint (_) => Some (EnumValue :: Unsigned (value . to_uint (value . size ()))) , _ => None , } } else { None } }
};
}
