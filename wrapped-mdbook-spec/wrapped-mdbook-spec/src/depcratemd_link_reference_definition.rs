// Generated macro for MD_LINK_REFERENCE_DEFINITION (static)
macro_rules! DepcrateMD_LINK_REFERENCE_DEFINITION {
() => {
// Module: crate
// Provides: {"MD_LINK_REFERENCE_DEFINITION"}
// Dependencies: {}
# [doc = " A primitive regex to find link reference definitions."] static MD_LINK_REFERENCE_DEFINITION : Lazy < Regex > = Lazy :: new (| | Regex :: new (r"(?m)^\[(?<label>[^]]+)]: +(?<dest>.*)") . unwrap ()) ;
};
}
