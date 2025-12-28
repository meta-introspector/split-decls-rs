macro_rules! ConflictType {
    () => {
        # [doc = " Constants passed to the conflict handler"] # [doc = " See [here](https://sqlite.org/session.html#SQLITE_CHANGESET_CONFLICT) for details."] # [allow (missing_docs)] # [repr (i32)] # [derive (Debug , PartialEq , Eq)] # [non_exhaustive] pub enum ConflictType { UNKNOWN = - 1 , SQLITE_CHANGESET_DATA = ffi :: SQLITE_CHANGESET_DATA , SQLITE_CHANGESET_NOTFOUND = ffi :: SQLITE_CHANGESET_NOTFOUND , SQLITE_CHANGESET_CONFLICT = ffi :: SQLITE_CHANGESET_CONFLICT , SQLITE_CHANGESET_CONSTRAINT = ffi :: SQLITE_CHANGESET_CONSTRAINT , SQLITE_CHANGESET_FOREIGN_KEY = ffi :: SQLITE_CHANGESET_FOREIGN_KEY , }
    };
}

ConflictType!()