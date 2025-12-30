// Generated macro for MontyFieldElement (struct)
macro_rules! Depcrate_montyMontyFieldElement {
() => {
// Module: crate::monty
// Provides: {"MontyFieldElement"}
// Dependencies: {}
# [doc = " Field element type which uses an internal Montgomery form representation."] # [derive (Clone , Copy)] pub struct MontyFieldElement < MOD , const LIMBS : usize > where MOD : MontyFieldParams < LIMBS > , { inner : MontyForm < MOD , LIMBS > , }
};
}
