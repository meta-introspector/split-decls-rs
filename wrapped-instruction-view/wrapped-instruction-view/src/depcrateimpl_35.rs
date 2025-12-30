// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a > InstructionAccount < 'a > { # [doc = " Creates a new `InstructionAccount`."] # [inline (always)] pub const fn new (address : & 'a Address , is_writable : bool , is_signer : bool) -> Self { Self { address , is_writable , is_signer , } } # [doc = " Creates a new read-only `InstructionAccount`."] # [inline (always)] pub const fn readonly (address : & 'a Address) -> Self { Self :: new (address , false , false) } # [doc = " Creates a new writable `InstructionAccount`."] # [inline (always)] pub const fn writable (address : & 'a Address) -> Self { Self :: new (address , true , false) } # [doc = " Creates a new read-only and signer `InstructionAccount`."] # [inline (always)] pub const fn readonly_signer (address : & 'a Address) -> Self { Self :: new (address , false , true) } # [doc = " Creates a new writable and signer `InstructionAccount`."] # [inline (always)] pub const fn writable_signer (address : & 'a Address) -> Self { Self :: new (address , true , true) } }
};
}
