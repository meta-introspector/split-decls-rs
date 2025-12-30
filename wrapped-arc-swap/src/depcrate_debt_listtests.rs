// Generated macro for tests (module)
macro_rules! Depcrate_debt_listtests {
() => {
// Module: crate::debt::list
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; impl Node { fn is_empty (& self) -> bool { self . fast_slots () . chain (core :: iter :: once (self . helping_slot ())) . all (| d | d . 0 . load (Relaxed) == Debt :: NONE) } fn get_thread () -> & 'static Self { LocalNode :: with (| h | h . node . get () . unwrap ()) } } # [doc = " A freshly acquired thread local node is empty."] # [test] fn new_empty () { assert ! (Node :: get_thread () . is_empty ()) ; } }
};
}
