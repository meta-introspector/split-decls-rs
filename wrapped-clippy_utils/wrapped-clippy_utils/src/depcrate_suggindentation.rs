// Generated macro for indentation (function)
macro_rules! Depcrate_suggindentation {
() => {
// Module: crate::sugg
// Provides: {"indentation"}
// Dependencies: {}
# [doc = " Returns the indentation before `span` if there are nothing but `[ \\t]`"] # [doc = " before it on its line."] fn indentation < T : LintContext > (cx : & T , span : Span) -> Option < String > { let lo = cx . sess () . source_map () . lookup_char_pos (span . lo ()) ; lo . file . get_line (lo . line - 1) . and_then (| line | { if let Some ((pos , _)) = line . char_indices () . find (| & (_ , c) | c != ' ' && c != '\t') { if lo . col == CharPos (pos) { Some (line [.. pos] . into ()) } else { None } } else { None } }) }
};
}
