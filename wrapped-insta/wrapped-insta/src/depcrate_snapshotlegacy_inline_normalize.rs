// Generated macro for legacy_inline_normalize (function)
macro_rules! Depcrate_snapshotlegacy_inline_normalize {
() => {
// Module: crate::snapshot
// Provides: {"legacy_inline_normalize"}
// Dependencies: {}
# [doc = " legacy format - retain so old snapshots still work"] fn legacy_inline_normalize (frozen_value : & str) -> String { if ! frozen_value . trim_start () . starts_with ('⋮') { return frozen_value . to_string () ; } let mut buf = String :: new () ; let mut line_iter = frozen_value . lines () ; let mut indentation = 0 ; for line in & mut line_iter { let line_trimmed = line . trim_start () ; if line_trimmed . is_empty () { continue ; } indentation = line . len () - line_trimmed . len () ; buf . push_str (& line_trimmed [3 ..]) ; buf . push ('\n') ; break ; } for line in & mut line_iter { if let Some (prefix) = line . get (.. indentation) { if ! prefix . trim () . is_empty () { return "" . to_string () ; } } if let Some (remainder) = line . get (indentation ..) { if let Some (rest) = remainder . strip_prefix ('⋮') { buf . push_str (rest) ; buf . push ('\n') ; } else if remainder . trim () . is_empty () { continue ; } else { return "" . to_string () ; } } } buf . trim_end () . to_string () }
};
}
