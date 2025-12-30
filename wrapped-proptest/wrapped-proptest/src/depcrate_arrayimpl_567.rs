// Generated macro for impl_567 (impl)
macro_rules! Depcrate_arrayimpl_567 {
() => {
// Module: crate::array
// Provides: {"impl_567"}
// Dependencies: {}
impl < T : ValueTree , const N : usize > ValueTree for ArrayValueTree < [T ; N] > { type Value = [T :: Value ; N] ; fn current (& self) -> [T :: Value ; N] { core :: array :: from_fn (| i | self . tree [i] . current ()) } fn simplify (& mut self) -> bool { while self . shrinker < N { if self . tree [self . shrinker] . simplify () { self . last_shrinker = Some (self . shrinker) ; return true ; } else { self . shrinker += 1 ; } } false } fn complicate (& mut self) -> bool { if let Some (shrinker) = self . last_shrinker { self . shrinker = shrinker ; if self . tree [shrinker] . complicate () { true } else { self . last_shrinker = None ; false } } else { false } } }
};
}
