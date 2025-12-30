// Generated macro for Node (enum)
macro_rules! Depcrate_format_argsNode {
() => {
// Module: crate::format_args
// Provides: {"Node"}
// Dependencies: {}
# [doc = " A node inside a format string. The two variants `Text` and `Placeholder` represent the usual"] # [doc = " kind of nodes that can be found in `format!`-like macros."] # [doc = ""] # [doc = " E.g., the format string `\"Hello, {:?}, happy day\"` will have 3 nodes:"] # [doc = "  - `Text(\"Hello, \")`,"] # [doc = "  - `Placeholder(\"{:?}\")`,"] # [doc = "  - `Text(\", happy day\")`."] # [doc = ""] # [doc = " The third kind of node: `Color(&str)`, represents a color code to apply."] # [doc = ""] # [doc = " E.g., the format string `\"This is a <blue>{}<clear> idea\"` will have 7 nodes:"] # [doc = "  - `Text(\"This is a \")`"] # [doc = "  - `Color(\"blue\")`"] # [doc = "  - `Placeholder(\"{}\")`"] # [doc = "  - `Color(\"clear\")`"] # [doc = "  - `Text(\" idea\")`"] # [derive (Debug)] pub enum Node < 'a > { Text (& 'a str) , Placeholder (& 'a str) , ColorTagGroup (Vec < ColorTag < 'a > >) , }
};
}
