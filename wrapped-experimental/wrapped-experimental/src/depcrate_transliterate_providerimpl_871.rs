// Generated macro for impl_871 (impl)
macro_rules! Depcrate_transliterate_providerimpl_871 {
() => {
// Module: crate::transliterate::provider
// Provides: {"impl_871"}
// Dependencies: {}
impl VarTable < '_ > { # [doc = " The lowest `char` used for encoding specials."] pub const BASE : char = '\u{F0000}' ; # [doc = " The highest `char` used for encoding dynamic (i.e., growing, non-reserved) specials."] pub const MAX_DYNAMIC : char = '\u{FFFF0}' ; # [doc = " The `char` that encodes a pure cursor, `|` without `@`."] pub const RESERVED_PURE_CURSOR : char = '\u{FFFFB}' ; # [doc = " The `char` that encodes a start anchor, `^`."] pub const RESERVED_ANCHOR_START : char = '\u{FFFFC}' ; # [doc = " The `char` that encodes an end anchor, `$`."] pub const RESERVED_ANCHOR_END : char = '\u{FFFFD}' ; # [doc = " The range used for encoded specials."] pub const ENCODE_RANGE : RangeInclusive < char > = Self :: BASE ..= Self :: RESERVED_ANCHOR_END ; # [doc = " The number of `char`s available for encoding dynamic (i.e., growing, non-reserved) specials."] pub const NUM_DYNAMIC : usize = Self :: MAX_DYNAMIC as usize - Self :: BASE as usize + 1 ; }
};
}
