macro_rules! Fallback_AD {
    () => {
        # [cfg (not (llvm_enzyme))] pub (crate) mod Fallback_AD { # ! [allow (unused_variables)] pub (crate) fn set_inline (val : bool) { unimplemented ! () } pub (crate) fn set_print_perf (print : bool) { unimplemented ! () } pub (crate) fn set_print_activity (print : bool) { unimplemented ! () } pub (crate) fn set_print_type (print : bool) { unimplemented ! () } pub (crate) fn set_print_type_fun (fun_name : & str) { unimplemented ! () } pub (crate) fn set_print (print : bool) { unimplemented ! () } pub (crate) fn set_strict_aliasing (strict : bool) { unimplemented ! () } pub (crate) fn set_loose_types (loose : bool) { unimplemented ! () } pub (crate) fn set_rust_rules (val : bool) { unimplemented ! () } }
    };
}

Fallback_AD!();