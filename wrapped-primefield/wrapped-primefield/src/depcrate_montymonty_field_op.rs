// Generated macro for monty_field_op (macro)
macro_rules! Depcrate_montymonty_field_op {
() => {
// Module: crate::monty
// Provides: {"monty_field_op"}
// Dependencies: {}
# [doc = " Emit a `core::ops` trait wrapper for an inherent method."] macro_rules ! monty_field_op { ($ op : tt , $ func : ident , $ inner_func : ident) => { impl < MOD , const LIMBS : usize > $ op for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS >, { type Output = MontyFieldElement < MOD , LIMBS >; # [inline] fn $ func (self , rhs : MontyFieldElement < MOD , LIMBS >) -> MontyFieldElement < MOD , LIMBS > { < MontyFieldElement < MOD , LIMBS >>::$ inner_func (& self , & rhs) } } impl < MOD , const LIMBS : usize > $ op <& Self > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS >, { type Output = MontyFieldElement < MOD , LIMBS >; # [inline] fn $ func (self , rhs : & MontyFieldElement < MOD , LIMBS >) -> MontyFieldElement < MOD , LIMBS > { < MontyFieldElement < MOD , LIMBS >>::$ inner_func (& self , rhs) } } impl < MOD , const LIMBS : usize > $ op < Self > for & MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS >, { type Output = MontyFieldElement < MOD , LIMBS >; # [inline] fn $ func (self , rhs : & MontyFieldElement < MOD , LIMBS >) -> MontyFieldElement < MOD , LIMBS > { < MontyFieldElement < MOD , LIMBS >>::$ inner_func (self , rhs) } } } ; }
};
}
