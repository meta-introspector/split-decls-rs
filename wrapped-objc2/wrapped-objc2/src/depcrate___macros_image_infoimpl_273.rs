// Generated macro for impl_273 (impl)
macro_rules! Depcrate___macros_image_infoimpl_273 {
() => {
// Module: crate::__macros::image_info
// Provides: {"impl_273"}
// Dependencies: {}
# [allow (unused)] impl ImageInfo { # [doc = " Unused"] const FIX_AND_CONTINUE : u32 = 1 << 0 ; const SUPPORTS_GARBAGE_COLLECTED : u32 = 1 << 1 ; const REQUIRES_GARBAGE_COLLECTION : u32 = 1 << 2 ; const OPTIMIZED_BY_DYLD : u32 = 1 << 3 ; # [doc = " Unused"] const CORRECTED_SYNTHESIZE : u32 = 1 << 4 ; # [doc = " Whether we're compiling this to run on a simulator."] const IMAGE_IS_SIMULATED : u32 = 1 << 5 ; # [doc = " Whether we are generating class properties."] const CLASS_PROPERTIES : u32 = 1 << 6 ; const DYLD_PREOPTIMIZED : u32 = 1 << 7 ; const SWIFT_ABI_VERSION_SHIFT : u32 = 8 ; const SWIFT_ABI_VERSION_MASK : u32 = 0xff << Self :: SWIFT_ABI_VERSION_SHIFT ; const SWIFT_MINOR_VERSION_SHIFT : u32 = 16 ; const SWIFT_MINOR_VERSION_MASK : u32 = 0xff << Self :: SWIFT_MINOR_VERSION_SHIFT ; const SWIFT_MAJOR_VERSION_SHIFT : u32 = 24 ; const SWIFT_MAJOR_VERSION_MASK : u32 = 0xff << Self :: SWIFT_MAJOR_VERSION_SHIFT ; # [doc = " Fetches the image info for the current runtime + target combination"] # [inline] pub const fn system () -> Self { let mut flags = Self :: CLASS_PROPERTIES ; if cfg ! (target_simulator) { flags |= Self :: IMAGE_IS_SIMULATED ; } Self { version : 0 , flags } } }
};
}
