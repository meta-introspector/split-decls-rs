// Generated macro for is_concretize (function)
macro_rules! Depcrateis_concretize {
() => {
// Module: crate
// Provides: {"is_concretize"}
// Dependencies: {}
# [doc = " Does this Attribute represent Mockall's \"concretize\" pseudo-attribute?"] fn is_concretize (attr : & Attribute) -> bool { if attr . path () . segments . last () . unwrap () . ident == "concretize" { true } else if attr . path () . is_ident ("cfg_attr") { match & attr . meta { Meta :: List (ml) => { ml . tokens . to_string () . contains ("concretize") } , _ => false , } } else { false } }
};
}
