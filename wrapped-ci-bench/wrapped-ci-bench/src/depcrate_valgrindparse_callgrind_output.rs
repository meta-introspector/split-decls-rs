// Generated macro for parse_callgrind_output (function)
macro_rules! Depcrate_valgrindparse_callgrind_output {
() => {
// Module: crate::valgrind
// Provides: {"parse_callgrind_output"}
// Dependencies: {}
# [doc = " Returns the instruction count, extracted from the callgrind output file at the provided path"] fn parse_callgrind_output (file : & Path) -> anyhow :: Result < u64 > { let file_in = File :: open (file) . context ("Unable to open callgrind output file") ? ; for line in BufReader :: new (file_in) . lines () { let line = line . context ("Error reading callgrind output file") ? ; if let Some (line) = line . strip_prefix ("summary: ") { let instr_count = line . trim () . parse () . context ("Unable to parse instruction counts from callgrind output file") ? ; return Ok (instr_count) ; } } anyhow :: bail ! ("`summary` section not found in callgrind output file") }
};
}
