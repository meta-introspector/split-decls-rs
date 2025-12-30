// Generated macro for ADMONITION_RE (static)
macro_rules! Depcrate_admonitionsADMONITION_RE {
() => {
// Module: crate::admonitions
// Provides: {"ADMONITION_RE"}
// Dependencies: {}
# [doc = " The Regex for the syntax for blockquotes that have a specific CSS class,"] # [doc = " like `> [!WARNING]`."] static ADMONITION_RE : LazyLock < Regex > = LazyLock :: new (| | { Regex :: new (r"(?m)^ *> \[!(?<admon>[^]]+)\]\n(?<blockquote>(?: *>.*\n)+)") . unwrap () }) ;
};
}
