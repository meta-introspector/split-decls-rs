// Generated macro for GENERATION_NUMBER_MAX (const)
macro_rules! DepcrateGENERATION_NUMBER_MAX {
() => {
// Module: crate
// Provides: {"GENERATION_NUMBER_MAX"}
// Dependencies: {}
# [doc = " The largest valid generation number."] # [doc = ""] # [doc = " If a commit's real generation number is larger than this, the commit graph will cap the value to"] # [doc = " this number."] # [doc = " The largest distinct generation number is `GENERATION_NUMBER_MAX - 1`."] pub const GENERATION_NUMBER_MAX : u32 = 0x3fff_ffff ;
};
}
