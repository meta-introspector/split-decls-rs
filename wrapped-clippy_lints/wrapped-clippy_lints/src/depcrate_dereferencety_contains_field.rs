// Generated macro for ty_contains_field (function)
macro_rules! Depcrate_dereferencety_contains_field {
() => {
// Module: crate::dereference
// Provides: {"ty_contains_field"}
// Dependencies: {}
fn ty_contains_field (ty : Ty < '_ > , name : Symbol) -> bool { if let ty :: Adt (adt , _) = * ty . kind () { adt . is_struct () && adt . all_fields () . any (| f | f . name == name) } else { false } }
};
}
