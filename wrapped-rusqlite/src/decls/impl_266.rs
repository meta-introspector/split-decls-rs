macro_rules! deps {
    () => {
        ConflictType!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl From < i32 > for ConflictType { fn from (code : i32) -> ConflictType { match code { ffi :: SQLITE_CHANGESET_DATA => ConflictType :: SQLITE_CHANGESET_DATA , ffi :: SQLITE_CHANGESET_NOTFOUND => ConflictType :: SQLITE_CHANGESET_NOTFOUND , ffi :: SQLITE_CHANGESET_CONFLICT => ConflictType :: SQLITE_CHANGESET_CONFLICT , ffi :: SQLITE_CHANGESET_CONSTRAINT => ConflictType :: SQLITE_CHANGESET_CONSTRAINT , ffi :: SQLITE_CHANGESET_FOREIGN_KEY => ConflictType :: SQLITE_CHANGESET_FOREIGN_KEY , _ => ConflictType :: UNKNOWN , } } }
    };
}

impl_266!();