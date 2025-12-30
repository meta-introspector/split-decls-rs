// Generated macro for ALL_OPERATIONS (static)
macro_rules! Depcrate_sharedALL_OPERATIONS {
() => {
// Module: crate::shared
// Provides: {"ALL_OPERATIONS"}
// Dependencies: {}
# [doc = " A flat representation of `ALL_FUNCTIONS`."] pub static ALL_OPERATIONS : LazyLock < Vec < MathOpInfo > > = LazyLock :: new (| | { let mut ret = Vec :: new () ; for op in ALL_OPERATIONS_NESTED { let fn_names = op . fn_list ; for name in fn_names { let api = MathOpInfo { name , float_ty : op . float_ty , rust_sig : op . rust_sig . clone () , c_sig : op . c_sig . clone () . unwrap_or_else (| | op . rust_sig . clone ()) , public : op . public , } ; ret . push (api) ; } if ! fn_names . is_sorted () { let mut sorted = (* fn_names) . to_owned () ; sorted . sort_unstable () ; panic ! ("names list is not sorted: {fn_names:?}\nExpected: {sorted:?}") ; } } ret . sort_by_key (| item | item . name) ; ret }) ;
};
}
