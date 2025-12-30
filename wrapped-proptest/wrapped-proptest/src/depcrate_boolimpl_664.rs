// Generated macro for impl_664 (impl)
macro_rules! Depcrate_boolimpl_664 {
() => {
// Module: crate::bool
// Provides: {"impl_664"}
// Dependencies: {}
impl ValueTree for BoolValueTree { type Value = bool ; fn current (& self) -> bool { self . current } fn simplify (& mut self) -> bool { match self . state { ShrinkState :: Untouched if self . current => { self . current = false ; self . state = ShrinkState :: Simplified ; true } ShrinkState :: Untouched | ShrinkState :: Simplified | ShrinkState :: Final => { self . state = ShrinkState :: Final ; false } } } fn complicate (& mut self) -> bool { match self . state { ShrinkState :: Untouched | ShrinkState :: Final => { self . state = ShrinkState :: Final ; false } ShrinkState :: Simplified => { self . current = true ; self . state = ShrinkState :: Final ; true } } } }
};
}
