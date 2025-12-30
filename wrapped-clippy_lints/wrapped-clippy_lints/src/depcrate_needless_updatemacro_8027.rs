// Generated macro for macro_8027 (macro)
macro_rules! Depcrate_needless_updatemacro_8027 {
() => {
// Module: crate::needless_update
// Provides: {"macro_8027"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for needlessly including a base struct on update"] # [doc = " when all fields are changed anyway."] # [doc = ""] # [doc = " This lint is not applied to structs marked with"] # [doc = " [non_exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This will cost resources (because the base has to be"] # [doc = " somewhere), and make the code less readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # struct Point {"] # [doc = " #     x: i32,"] # [doc = " #     y: i32,"] # [doc = " #     z: i32,"] # [doc = " # }"] # [doc = " # let zero_point = Point { x: 0, y: 0, z: 0 };"] # [doc = " Point {"] # [doc = "     x: 1,"] # [doc = "     y: 1,"] # [doc = "     z: 1,"] # [doc = "     ..zero_point"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " // Missing field `z`"] # [doc = " Point {"] # [doc = "     x: 1,"] # [doc = "     y: 1,"] # [doc = "     ..zero_point"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEEDLESS_UPDATE , complexity , "using `Foo { ..base }` when there are no missing fields" }
};
}
