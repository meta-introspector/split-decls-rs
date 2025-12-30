// Generated macro for impl_76 (impl)
macro_rules! Depcrate_typesimpl_76 {
() => {
// Module: crate::types
// Provides: {"impl_76"}
// Dependencies: {}
impl < T > Selection < T > { fn is_negated (& self) -> bool { match * self { Selection :: Select (..) => false , Selection :: Negate (..) => true , } } fn name (& self) -> & str { match * self { Selection :: Select (ref name , _) => name , Selection :: Negate (ref name , _) => name , } } fn map < U , F : FnOnce (T) -> U > (self , f : F) -> Selection < U > { match self { Selection :: Select (name , inner) => { Selection :: Select (name , f (inner)) } Selection :: Negate (name , inner) => { Selection :: Negate (name , f (inner)) } } } fn inner (& self) -> & T { match * self { Selection :: Select (_ , ref inner) => inner , Selection :: Negate (_ , ref inner) => inner , } } }
};
}
