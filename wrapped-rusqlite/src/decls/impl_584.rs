macro_rules! deps {
    () => {
        ConflictMode!();
    };
}

macro_rules! impl_584 {
    () => {
        deps!();
        impl From < c_int > for ConflictMode { fn from (value : c_int) -> Self { match value { ffi :: SQLITE_ROLLBACK => ConflictMode :: Rollback , ffi :: SQLITE_IGNORE => ConflictMode :: Ignore , ffi :: SQLITE_FAIL => ConflictMode :: Fail , ffi :: SQLITE_ABORT => ConflictMode :: Abort , ffi :: SQLITE_REPLACE => ConflictMode :: Replace , _ => unreachable ! ("sqlite3_vtab_on_conflict returned invalid value") , } } }
    };
}

impl_584!();