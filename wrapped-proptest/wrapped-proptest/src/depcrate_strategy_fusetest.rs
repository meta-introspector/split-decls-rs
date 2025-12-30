// Generated macro for test (module)
macro_rules! Depcrate_strategy_fusetest {
() => {
// Module: crate::strategy::fuse
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; struct StrictValueTree { min : u32 , curr : u32 , max : u32 , ready : bool , } impl StrictValueTree { fn new (start : u32) -> Self { StrictValueTree { min : 0 , curr : start , max : start , ready : false , } } } impl ValueTree for StrictValueTree { type Value = u32 ; fn current (& self) -> u32 { self . curr } fn simplify (& mut self) -> bool { assert ! (self . min <= self . curr) ; if self . curr > self . min { self . max = self . curr ; self . curr -= 1 ; self . ready = true ; true } else { self . min += 1 ; false } } fn complicate (& mut self) -> bool { assert ! (self . max >= self . curr) ; assert ! (self . ready) ; if self . curr < self . max { self . curr += 1 ; true } else { self . max -= 1 ; false } } } # [test] fn test_sanity () { check_strategy_sanity (Fuse :: new (0i32 .. 100i32) , None) ; } # [test] fn guards_bad_transitions () { let mut vt = Fuse :: new (StrictValueTree :: new (5)) ; assert ! (! vt . complicate ()) ; assert_eq ! (5 , vt . current ()) ; assert ! (vt . simplify ()) ; assert ! (vt . simplify ()) ; assert ! (vt . simplify ()) ; assert ! (vt . simplify ()) ; assert ! (vt . simplify ()) ; assert_eq ! (0 , vt . current ()) ; assert ! (! vt . simplify ()) ; assert ! (! vt . simplify ()) ; assert_eq ! (0 , vt . current ()) ; assert ! (vt . complicate ()) ; assert_eq ! (1 , vt . current ()) ; assert ! (! vt . complicate ()) ; assert ! (! vt . complicate ()) ; assert_eq ! (1 , vt . current ()) ; } }
};
}
