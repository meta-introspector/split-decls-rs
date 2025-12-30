// Generated macro for is_outer_attr (function)
macro_rules! Depcrate_attris_outer_attr {
() => {
// Module: crate::attr
// Provides: {"is_outer_attr"}
// Dependencies: {}
# [doc = " Returns true iff the given attribute is an outer one, i.e: `#[<attr>]`."] # [doc = " An inner attribute is the other possibility and has the syntax `#![<attr>]`."] # [doc = " Note that `<attr>` is a meta-variable for the contents inside."] fn is_outer_attr (attr : & Attribute) -> bool { syn :: AttrStyle :: Outer == attr . style }
};
}
