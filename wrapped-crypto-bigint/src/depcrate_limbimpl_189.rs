// Generated macro for impl_189 (impl)
macro_rules! Depcrate_limbimpl_189 {
() => {
// Module: crate::limb
// Provides: {"impl_189"}
// Dependencies: {}
impl Limb { # [doc = " The value `0`."] pub const ZERO : Self = Limb (0) ; # [doc = " The value `1`."] pub const ONE : Self = Limb (1) ; # [doc = " Maximum value this [`Limb`] can express."] pub const MAX : Self = Limb (Word :: MAX) ; # [doc = " Highest bit in a [`Limb`]."] pub (crate) const HI_BIT : u32 = Limb :: BITS - 1 ; # [doc = " Size of the inner integer in bits."] # [cfg (target_pointer_width = "32")] pub const BITS : u32 = 32 ; # [doc = " Size of the inner integer in bytes."] # [cfg (target_pointer_width = "32")] pub const BYTES : usize = 4 ; # [doc = " Size of the inner integer in bits."] # [cfg (target_pointer_width = "64")] pub const BITS : u32 = 64 ; # [doc = " Size of the inner integer in bytes."] # [cfg (target_pointer_width = "64")] pub const BYTES : usize = 8 ; # [doc = " `floor(log2(Self::BITS))`."] pub const LOG2_BITS : u32 = u32 :: BITS - (Self :: BITS - 1) . leading_zeros () ; # [doc = " Convert to a [`NonZero<Limb>`]."] # [doc = ""] # [doc = " Returns some if the original value is non-zero, and false otherwise."] pub const fn to_nz (self) -> ConstCtOption < NonZero < Self > > { ConstCtOption :: new (NonZero (self) , self . is_nonzero ()) } }
};
}
