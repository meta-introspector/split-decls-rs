// Generated macro for parse_covmap_line (function)
macro_rules! Depcrate_covmapparse_covmap_line {
() => {
// Module: crate::covmap
// Provides: {"parse_covmap_line"}
// Dependencies: {}
fn parse_covmap_line (line : & str) -> anyhow :: Result < CovmapLineData > { ensure ! (is_covmap_line (line)) ; const RE_STRING : & str = r#"(?x)^
        @__llvm_coverage_mapping \ =
        .*
        \[ [0-9]+ \ x \ i8 \] \ c"(?<payload>[^"]*)"
        .*$
    "# ; static RE : LazyLock < Regex > = LazyLock :: new (| | Regex :: new (RE_STRING) . unwrap ()) ; let captures = RE . captures (line) . with_context (| | format ! ("couldn't parse covmap line: {line:?}")) ? ; let payload = unescape_llvm_string_contents (& captures ["payload"]) ; Ok (CovmapLineData { payload }) }
};
}
