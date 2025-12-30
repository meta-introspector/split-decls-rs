// Generated macro for impl_76 (impl)
macro_rules! Depcrate_blockimpl_76 {
() => {
// Module: crate::block
// Provides: {"impl_76"}
// Dependencies: {}
impl Block { pub const USIZE_COUNT : usize = core :: mem :: size_of :: < Self > () / core :: mem :: size_of :: < usize > () ; pub const NONE : Self = Self :: from_usize_array ([0 ; Self :: USIZE_COUNT]) ; pub const ALL : Self = Self :: from_usize_array ([usize :: MAX ; Self :: USIZE_COUNT]) ; pub const BITS : usize = core :: mem :: size_of :: < Self > () * 8 ; # [inline] pub fn into_usize_array (self) -> [usize ; Self :: USIZE_COUNT] { unsafe { core :: mem :: transmute (self . 0) } } # [inline] pub const fn from_usize_array (array : [usize ; Self :: USIZE_COUNT]) -> Self { Self (unsafe { core :: mem :: transmute (array) }) } }
};
}
