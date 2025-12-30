// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl List { fn tail (& self) -> Option < & RefCell < Rc < List > > > { match self { Cons (_ , item) => Some (item) , Nil => None , } } }
};
}
