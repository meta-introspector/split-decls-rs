// Generated macro for TzifFixed (struct)
macro_rules! Depcrate_sharedTzifFixed {
() => {
// Module: crate::shared
// Provides: {"TzifFixed"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct TzifFixed < STR , ABBREV > { pub name : Option < STR > , # [doc = " An ASCII byte corresponding to the version number. So, 0x50 is '2'."] # [doc = ""] # [doc = " This is unused. It's only used in `test` compilation for emitting"] # [doc = " diagnostic data about TZif files. If we really need to use this, we"] # [doc = " should probably just convert it to an actual integer."] pub version : u8 , pub checksum : u32 , pub designations : STR , pub posix_tz : Option < PosixTimeZone < ABBREV > > , }
};
}
