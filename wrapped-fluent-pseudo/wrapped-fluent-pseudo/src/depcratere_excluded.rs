// Generated macro for RE_EXCLUDED (static)
macro_rules! DepcrateRE_EXCLUDED {
() => {
// Module: crate
// Provides: {"RE_EXCLUDED"}
// Dependencies: {}
static RE_EXCLUDED : Lazy < Regex > = Lazy :: new (| | Regex :: new (r"&[#\w]+;|<\s*.+?\s*>") . unwrap ()) ;
};
}
