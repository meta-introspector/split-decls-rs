// Generated macro for recv (function)
macro_rules! Depcraterecv {
() => {
// Module: crate
// Provides: {"recv"}
// Dependencies: {}
fn recv < B : BufRead > (dir : & Path , io : & mut B) -> PathBuf { let mut filename = Vec :: new () ; t ! (io . read_until (0 , & mut filename)) ; let len = cmp :: min (filename . len () - 1 , 50) ; let dst = dir . join (t ! (str :: from_utf8 (& filename [.. len]))) ; let amt = read_u64 (io) ; t ! (io :: copy (& mut io . take (amt) , & mut t ! (File :: create (& dst)))) ; set_permissions (& dst) ; dst }
};
}
