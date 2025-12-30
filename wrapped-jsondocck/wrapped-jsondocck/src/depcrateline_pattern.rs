// Generated macro for LINE_PATTERN (static)
macro_rules! DepcrateLINE_PATTERN {
() => {
// Module: crate
// Provides: {"LINE_PATTERN"}
// Dependencies: {}
static LINE_PATTERN : LazyLock < Regex > = LazyLock :: new (| | { RegexBuilder :: new (r#"
        ^\s*
        //@\s+
        (?P<negated>!?)
        (?P<directive>.+?)
        (?:[\s:](?P<args>.*))?$
    "# ,) . ignore_whitespace (true) . unicode (true) . build () . unwrap () }) ;
};
}
