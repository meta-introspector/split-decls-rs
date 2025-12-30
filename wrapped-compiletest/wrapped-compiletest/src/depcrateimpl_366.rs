// Generated macro for impl_366 (impl)
macro_rules! Depcrateimpl_366 {
() => {
// Module: crate
// Provides: {"impl_366"}
// Dependencies: {}
impl TestCollector { fn new () -> Self { TestCollector { tests : vec ! [] , found_path_stems : HashSet :: new () , poisoned : false } } fn merge (& mut self , mut other : Self) { self . tests . append (& mut other . tests) ; self . found_path_stems . extend (other . found_path_stems) ; self . poisoned |= other . poisoned ; } }
};
}
