// Generated macro for tests (module)
macro_rules! Depcrate_windows_term_colorstests {
() => {
// Module: crate::windows_term::colors
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn color_parsing () { let intense_color = "leading bytes \x1b[38;5;10m trailing bytes" ; let parsed = driver (parse_color , intense_color) . unwrap () ; assert_eq ! (parsed , (Intense :: Yes , Color :: Green , FgBg :: Foreground)) ; let normal_color = "leading bytes \x1b[40m trailing bytes" ; let parsed = driver (parse_color , normal_color) . unwrap () ; assert_eq ! (parsed , (Intense :: No , Color :: Black , FgBg :: Background)) ; } # [test] fn attr_parsing () { let attr = "leading bytes \x1b[1m trailing bytes" ; let parsed = driver (parse_attr , attr) . unwrap () ; assert_eq ! (parsed , b'1') ; } }
};
}
