// Generated macro for bit_relations_inherent_impls (macro)
macro_rules! Depcrate_bit_setbit_relations_inherent_impls {
() => {
// Module: crate::bit_set
// Provides: {"bit_relations_inherent_impls"}
// Dependencies: {}
macro_rules ! bit_relations_inherent_impls { () => { # [doc = " Sets `self = self | other` and returns `true` if `self` changed"] # [doc = " (i.e., if new bits were added)."] pub fn union < Rhs > (& mut self , other : & Rhs) -> bool where Self : BitRelations < Rhs >, { < Self as BitRelations < Rhs >>:: union (self , other) } # [doc = " Sets `self = self - other` and returns `true` if `self` changed."] # [doc = " (i.e., if any bits were removed)."] pub fn subtract < Rhs > (& mut self , other : & Rhs) -> bool where Self : BitRelations < Rhs >, { < Self as BitRelations < Rhs >>:: subtract (self , other) } # [doc = " Sets `self = self & other` and return `true` if `self` changed."] # [doc = " (i.e., if any bits were removed)."] pub fn intersect < Rhs > (& mut self , other : & Rhs) -> bool where Self : BitRelations < Rhs >, { < Self as BitRelations < Rhs >>:: intersect (self , other) } } ; }
};
}
