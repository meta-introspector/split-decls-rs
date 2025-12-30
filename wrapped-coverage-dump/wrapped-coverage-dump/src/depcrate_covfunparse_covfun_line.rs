// Generated macro for parse_covfun_line (function)
macro_rules! Depcrate_covfunparse_covfun_line {
() => {
// Module: crate::covfun
// Provides: {"parse_covfun_line"}
// Dependencies: {}
# [doc = " Given a line of LLVM IR assembly that should contain an `__llvm_covfun`"] # [doc = " entry, parses it to extract relevant data in a `CovfunLineData`."] fn parse_covfun_line (line : & str) -> anyhow :: Result < CovfunLineData > { ensure ! (is_covfun_line (line)) ; const RE_STRING : & str = r#"(?x)^
        @__covrec_[0-9A-Z]+(?<is_used>u)?
        \ = \ # (trailing space)
        .*
        <\{
            \ i64 \ (?<name_hash> -? [0-9]+),
            \ i32 \ -? [0-9]+, # (length of payload; currently unused)
            \ i64 \ -? [0-9]+, # (source hash; currently unused)
            \ i64 \ (?<filenames_hash> -? [0-9]+),
            \ \[ [0-9]+ \ x \ i8 \] \ c"(?<payload>[^"]*)"
            \ # (trailing space)
        }>
        .*$
    "# ; static RE : LazyLock < Regex > = LazyLock :: new (| | Regex :: new (RE_STRING) . unwrap ()) ; let captures = RE . captures (line) . with_context (| | format ! ("couldn't parse covfun line: {line:?}")) ? ; let is_used = captures . name ("is_used") . is_some () ; let name_hash = i64 :: from_str_radix (& captures ["name_hash"] , 10) . unwrap () as u64 ; let filenames_hash = i64 :: from_str_radix (& captures ["filenames_hash"] , 10) . unwrap () as u64 ; let payload = unescape_llvm_string_contents (& captures ["payload"]) ; Ok (CovfunLineData { is_used , name_hash , filenames_hash , payload }) }
};
}
