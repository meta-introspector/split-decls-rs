// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "blake3")] mod tests { use super :: * ; # [test] fn test_hashv () { let val = "gHiljKpq" ; let val_hash = hash (val . as_bytes ()) ; let ext = "lM890t" ; let ext_hash = hashv (& [val_hash . as_bytes () , ext . as_bytes ()]) ; let hash_ext = [& val_hash . to_bytes () , ext . as_bytes ()] . concat () ; assert ! (ext_hash == hash (& hash_ext)) ; } }
};
}
