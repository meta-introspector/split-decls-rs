// Generated macro for isa_constructor_32 (function)
macro_rules! Depcrate_isa_pulley_sharedisa_constructor_32 {
() => {
// Module: crate::isa::pulley_shared
// Provides: {"isa_constructor_32"}
// Dependencies: {}
fn isa_constructor_32 (triple : Triple , shared_flags : Flags , builder : & shared_settings :: Builder ,) -> CodegenResult < OwnedTargetIsa > { use crate :: settings :: Configurable ; let mut builder = builder . clone () ; builder . set ("pointer_width" , "pointer32") . unwrap () ; if triple . endianness () . unwrap () == target_lexicon :: Endianness :: Big { builder . enable ("big_endian") . unwrap () ; } let isa_flags = PulleyFlags :: new (& shared_flags , & builder) ; let backend = PulleyBackend :: < super :: pulley32 :: Pulley32 > :: new_with_flags (triple , shared_flags , isa_flags) ; Ok (backend . wrapped ()) }
};
}
