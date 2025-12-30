// Generated macro for required_hashes (function)
macro_rules! Depcrate_snapshotrequired_hashes {
() => {
// Module: crate::snapshot
// Provides: {"required_hashes"}
// Dependencies: {}
# [doc = " The number of `#` we need to surround a raw string literal with."] fn required_hashes (text : & str) -> usize { text . split ('"') . skip (1) . map (| s | s . chars () . take_while (| & c | c == '#') . count () + 1) . max () . unwrap_or_default () }
};
}
