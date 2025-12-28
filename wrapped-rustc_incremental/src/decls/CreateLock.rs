macro_rules! CreateLock {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_create_lock)] pub (crate) struct CreateLock < 'a > { pub lock_err : std :: io :: Error , pub session_dir : & 'a Path , # [note (incremental_lock_unsupported)] pub is_unsupported_lock : bool , # [help (incremental_cargo_help_1)] # [help (incremental_cargo_help_2)] pub is_cargo : bool , }
    };
}

CreateLock!()