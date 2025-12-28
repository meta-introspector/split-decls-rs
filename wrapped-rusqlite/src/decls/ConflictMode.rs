macro_rules! ConflictMode {
    () => {
        # [doc = " Conflict resolution modes"] # [non_exhaustive] # [derive (Debug , Eq , PartialEq)] pub enum ConflictMode { # [doc = " SQLITE_ROLLBACK"] Rollback , # [doc = " SQLITE_IGNORE"] Ignore , # [doc = " SQLITE_FAIL"] Fail , # [doc = " SQLITE_ABORT"] Abort , # [doc = " SQLITE_REPLACE"] Replace , }
    };
}

ConflictMode!()