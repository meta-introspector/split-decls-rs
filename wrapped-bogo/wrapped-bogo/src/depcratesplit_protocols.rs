// Generated macro for split_protocols (function)
macro_rules! Depcratesplit_protocols {
() => {
// Module: crate
// Provides: {"split_protocols"}
// Dependencies: {}
fn split_protocols (protos : & str) -> Vec < String > { let mut ret = Vec :: new () ; let mut offs = 0 ; while offs < protos . len () { let len = protos . as_bytes () [offs] as usize ; let item = protos [offs + 1 .. offs + 1 + len] . to_string () ; ret . push (item) ; offs += 1 + len ; } ret }
};
}
