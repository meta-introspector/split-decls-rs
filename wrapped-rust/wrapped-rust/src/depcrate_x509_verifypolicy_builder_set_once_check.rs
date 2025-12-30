// Generated macro for policy_builder_set_once_check (macro)
macro_rules! Depcrate_x509_verifypolicy_builder_set_once_check {
() => {
// Module: crate::x509::verify
// Provides: {"policy_builder_set_once_check"}
// Dependencies: {}
macro_rules ! policy_builder_set_once_check { ($ self : ident , $ property : ident , $ human_readable_name : literal) => { if $ self .$ property . is_some () { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (concat ! ("The " , $ human_readable_name , " may only be set once.")) ,)) ; } } ; }
};
}
