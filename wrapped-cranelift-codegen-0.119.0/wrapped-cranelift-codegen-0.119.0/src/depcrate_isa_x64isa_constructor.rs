// Generated macro for isa_constructor (function)
macro_rules! Depcrate_isa_x64isa_constructor {
() => {
// Module: crate::isa::x64
// Provides: {"isa_constructor"}
// Dependencies: {}
fn isa_constructor (triple : Triple , shared_flags : Flags , builder : & shared_settings :: Builder ,) -> CodegenResult < OwnedTargetIsa > { let isa_flags = x64_settings :: Flags :: new (& shared_flags , builder) ; let backend = X64Backend :: new_with_flags (triple , shared_flags , isa_flags) ; Ok (backend . wrapped ()) }
};
}
