// Generated macro for isa_builder (function)
macro_rules! Depcrate_isa_s390xisa_builder {
() => {
// Module: crate::isa::s390x
// Provides: {"isa_builder"}
// Dependencies: {}
# [doc = " Create a new `isa::Builder`."] pub fn isa_builder (triple : Triple) -> IsaBuilder { assert ! (triple . architecture == Architecture :: S390x) ; IsaBuilder { triple , setup : s390x_settings :: builder () , constructor : | triple , shared_flags , builder | { let isa_flags = s390x_settings :: Flags :: new (& shared_flags , builder) ; let backend = S390xBackend :: new_with_flags (triple , shared_flags , isa_flags) ; Ok (backend . wrapped ()) } , } }
};
}
