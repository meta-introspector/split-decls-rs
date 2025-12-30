// Generated macro for HasCrate (trait)
macro_rules! DepcrateHasCrate {
() => {
// Module: crate
// Provides: {"HasCrate"}
// Dependencies: {}
# [doc = " Trait for obtaining the defining crate of an item."] pub trait HasCrate { fn krate (& self , db : & dyn HirDatabase) -> Crate ; }
};
}
