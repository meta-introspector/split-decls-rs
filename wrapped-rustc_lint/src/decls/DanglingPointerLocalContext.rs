macro_rules! DanglingPointerLocalContext {
    () => {
        struct DanglingPointerLocalContext < 'tcx > { body : LocalDefId , fn_ret : Ty < 'tcx > , fn_ret_span : Span , fn_ret_inner : Ty < 'tcx > , fn_kind : & 'static str , }
    };
}

DanglingPointerLocalContext!();