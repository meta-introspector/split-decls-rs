// Generated macro for impl_218 (impl)
macro_rules! Depcrateimpl_218 {
() => {
// Module: crate
// Provides: {"impl_218"}
// Dependencies: {}
impl Plot { fn new < S > (data : Matrix , script : & S) -> Plot where S : Script , { Plot { data , script : script . script () , } } fn data (& self) -> & Matrix { & self . data } fn script (& self) -> & str { & self . script } }
};
}
