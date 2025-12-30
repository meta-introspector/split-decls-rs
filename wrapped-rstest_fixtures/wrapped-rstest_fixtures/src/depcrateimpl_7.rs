// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl < T , G : TearDown > Fixture < T , G > { pub fn new (inner : T , guard : G) -> Self { Fixture { inner : Some (inner) , guard : Some (guard) } } pub fn take (& mut self) -> T { self . inner . take () . unwrap () } pub fn guard (& mut self) -> G { self . guard . take () . unwrap () } pub fn compose < OTHER : TearDown > (mut self , guard : OTHER) -> Fixture < T , (G , OTHER) > { Fixture :: new (self . take () , (self . guard () , guard)) } }
};
}
