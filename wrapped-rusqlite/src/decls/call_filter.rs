macro_rules! deps {
    () => {
        ChangesetItem!();
        ConflictAction!();
        ConflictType!();
    };
}

macro_rules! call_filter {
    () => {
        deps!();
        unsafe extern "C" fn call_filter < F , C > (p_ctx : * mut c_void , tbl_str : * const c_char) -> c_int where F : Fn (& str) -> bool + Send + 'static , C : Fn (ConflictType , ChangesetItem) -> ConflictAction + Send + 'static , { let tbl_name = CStr :: from_ptr (tbl_str) . to_str () ; c_int :: from (catch_unwind (| | { let tuple : * mut (Option < F > , C) = p_ctx . cast :: < (Option < F > , C) > () ; if let Some (ref filter) = (* tuple) . 0 { filter (tbl_name . expect ("illegal table name")) } else { true } }) . unwrap_or_default () ,) }
    };
}

call_filter!();