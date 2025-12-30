// Generated macro for DEPRECATED_LINE_PATTERN (static)
macro_rules! DepcrateDEPRECATED_LINE_PATTERN {
() => {
// Module: crate
// Provides: {"DEPRECATED_LINE_PATTERN"}
// Dependencies: {}
static DEPRECATED_LINE_PATTERN : LazyLock < Regex > = LazyLock :: new (| | { RegexBuilder :: new (r"//\s+@") . ignore_whitespace (true) . unicode (true) . build () . unwrap () }) ;
};
}
