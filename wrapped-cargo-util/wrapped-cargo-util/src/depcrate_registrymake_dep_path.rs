// Generated macro for make_dep_path (function)
macro_rules! Depcrate_registrymake_dep_path {
() => {
// Module: crate::registry
// Provides: {"make_dep_path"}
// Dependencies: {}
# [doc = " Make a path to a dependency, which aligns to"] # [doc = ""] # [doc = " - [index from of Cargo's index on filesystem][1], and"] # [doc = " - [index from Crates.io][2]."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Note: For index files, `dep_name` must have had `to_lowercase` called on it."] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " [1]: https://docs.rs/cargo/latest/cargo/sources/registry/index.html#the-format-of-the-index"] # [doc = " [2]: https://github.com/rust-lang/crates.io-index"] pub fn make_dep_path (dep_name : & str , prefix_only : bool) -> String { let (slash , name) = if prefix_only { ("" , "") } else { ("/" , dep_name) } ; match dep_name . len () { 1 => format ! ("1{}{}" , slash , name) , 2 => format ! ("2{}{}" , slash , name) , 3 => format ! ("3/{}{}{}" , & dep_name [.. 1] , slash , name) , _ => format ! ("{}/{}{}{}" , & dep_name [0 .. 2] , & dep_name [2 .. 4] , slash , name) , } }
};
}
