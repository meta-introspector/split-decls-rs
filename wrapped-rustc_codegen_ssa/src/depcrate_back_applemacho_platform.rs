// Generated macro for macho_platform (function)
macro_rules! Depcrate_back_applemacho_platform {
() => {
// Module: crate::back::apple
// Provides: {"macho_platform"}
// Dependencies: {}
pub (super) fn macho_platform (target : & Target) -> u32 { match (& * target . os , & * target . env) { ("macos" , _) => object :: macho :: PLATFORM_MACOS , ("ios" , "macabi") => object :: macho :: PLATFORM_MACCATALYST , ("ios" , "sim") => object :: macho :: PLATFORM_IOSSIMULATOR , ("ios" , _) => object :: macho :: PLATFORM_IOS , ("watchos" , "sim") => object :: macho :: PLATFORM_WATCHOSSIMULATOR , ("watchos" , _) => object :: macho :: PLATFORM_WATCHOS , ("tvos" , "sim") => object :: macho :: PLATFORM_TVOSSIMULATOR , ("tvos" , _) => object :: macho :: PLATFORM_TVOS , ("visionos" , "sim") => object :: macho :: PLATFORM_XROSSIMULATOR , ("visionos" , _) => object :: macho :: PLATFORM_XROS , _ => unreachable ! ("tried to get Mach-O platform for non-Apple target") , } }
};
}
