// Generated macro for is_ty_param (function)
macro_rules! Depcrate_non_send_fields_in_send_tyis_ty_param {
() => {
// Module: crate::non_send_fields_in_send_ty
// Provides: {"is_ty_param"}
// Dependencies: {}
# [doc = " Returns `true` if the type is a type parameter such as `T`."] fn is_ty_param (target_ty : Ty < '_ >) -> bool { matches ! (target_ty . kind () , ty :: Param (_)) }
};
}
