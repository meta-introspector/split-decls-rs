// Generated macro for Discriminator (struct)
macro_rules! Depcrate_astDiscriminator {
() => {
// Module: crate::ast
// Provides: {"Discriminator"}
// Dependencies: {}
# [doc = " The `<discriminator>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <discriminator> := _ <non-negative number>      # when number < 10"] # [doc = "                 := __ <non-negative number> _   # when number >= 10"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Discriminator (usize) ;
};
}
