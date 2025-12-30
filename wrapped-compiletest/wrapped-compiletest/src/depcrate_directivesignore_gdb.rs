// Generated macro for ignore_gdb (function)
macro_rules! Depcrate_directivesignore_gdb {
() => {
// Module: crate::directives
// Provides: {"ignore_gdb"}
// Dependencies: {}
fn ignore_gdb (config : & Config , line : & str) -> IgnoreDecision { if config . debugger != Some (Debugger :: Gdb) { return IgnoreDecision :: Continue ; } if let Some (actual_version) = config . gdb_version { if let Some (rest) = line . strip_prefix ("min-gdb-version:") . map (str :: trim) { let (start_ver , end_ver) = extract_version_range (rest , extract_gdb_version) . unwrap_or_else (| | { panic ! ("couldn't parse version range: {:?}" , rest) ; }) ; if start_ver != end_ver { panic ! ("Expected single GDB version") } if actual_version < start_ver { return IgnoreDecision :: Ignore { reason : format ! ("ignored when the GDB version is lower than {rest}") , } ; } } else if let Some (rest) = line . strip_prefix ("ignore-gdb-version:") . map (str :: trim) { let (min_version , max_version) = extract_version_range (rest , extract_gdb_version) . unwrap_or_else (| | { panic ! ("couldn't parse version range: {:?}" , rest) ; }) ; if max_version < min_version { panic ! ("Malformed GDB version range: max < min") } if actual_version >= min_version && actual_version <= max_version { if min_version == max_version { return IgnoreDecision :: Ignore { reason : format ! ("ignored when the GDB version is {rest}") , } ; } else { return IgnoreDecision :: Ignore { reason : format ! ("ignored when the GDB version is between {rest}") , } ; } } } } IgnoreDecision :: Continue }
};
}
