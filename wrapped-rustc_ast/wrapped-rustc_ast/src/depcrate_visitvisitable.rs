// Generated macro for Visitable (trait)
macro_rules! Depcrate_visitVisitable {
() => {
// Module: crate::visit
// Provides: {"Visitable"}
// Dependencies: {}
pub (crate) trait Visitable < 'a , V : Visitor < 'a > > { type Extra : Copy ; # [must_use] fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result ; }
};
}
