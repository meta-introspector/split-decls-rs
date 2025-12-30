// Generated macro for methods_target_is_kind (macro)
macro_rules! Depcratemethods_target_is_kind {
() => {
// Module: crate
// Provides: {"methods_target_is_kind"}
// Dependencies: {}
macro_rules ! methods_target_is_kind { ($ ($ name : ident => $ kind : expr) ,*) => { $ (# [doc = " Return true if this target is of kind `$kind`."] pub fn $ name (& self) -> bool { self . is_kind ($ kind) }) * } }
};
}
