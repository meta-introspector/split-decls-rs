// Generated macro for tests (module)
macro_rules! Depcrate_debttests {
() => {
// Module: crate::debt
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use alloc :: sync :: Arc ; # [doc = " Checks the assumption that arcs to ZSTs have different pointer values."] # [test] fn arc_zst () { struct A ; struct B ; let a = Arc :: new (A) ; let b = Arc :: new (B) ; let aref : & A = & a ; let bref : & B = & b ; let aptr = aref as * const _ as usize ; let bptr = bref as * const _ as usize ; assert_ne ! (aptr , bptr) ; } }
};
}
