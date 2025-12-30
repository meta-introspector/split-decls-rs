// Generated macro for check_attrs_get_cratename (function)
macro_rules! Depcratecheck_attrs_get_cratename {
() => {
// Module: crate
// Provides: {"check_attrs_get_cratename"}
// Dependencies: {}
fn check_attrs_get_cratename (input : & TokenStream) -> Result < Path , Error > { let input = input . clone () ; let derive_input = syn :: parse :: < DeriveInput > (input) ? ; item :: check_attributes (& derive_input) ? ; cratename :: get (& derive_input . attrs) }
};
}
