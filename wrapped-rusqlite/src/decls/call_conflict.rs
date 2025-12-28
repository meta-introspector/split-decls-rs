macro_rules! deps {
    () => {
        ChangesetItem!();
        ConflictAction!();
        ConflictType!();
    };
}

macro_rules! call_conflict {
    () => {
        deps!();
        unsafe extern "C" fn call_conflict < F , C > (p_ctx : * mut c_void , e_conflict : c_int , p : * mut ffi :: sqlite3_changeset_iter ,) -> c_int where F : Fn (& str) -> bool + Send + 'static , C : Fn (ConflictType , ChangesetItem) -> ConflictAction + Send + 'static , { let conflict_type = ConflictType :: from (e_conflict) ; let item = ChangesetItem { it : p } ; if let Ok (action) = catch_unwind (| | { let tuple : * mut (Option < F > , C) = p_ctx . cast :: < (Option < F > , C) > () ; (* tuple) . 1 (conflict_type , item) }) { action as c_int } else { ffi :: SQLITE_CHANGESET_ABORT } }
    };
}

call_conflict!();