// Generated macro for macro_7029 (macro)
macro_rules! Depcrate_methodsmacro_7029 {
() => {
// Module: crate::methods
// Provides: {"macro_7029"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.collect::<Vec<String>>().join(\"\")` on iterators."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.collect::<String>()` is more concise and might be more performant"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let vector = vec![\"hello\",  \"world\"];"] # [doc = " let output = vector.iter().map(|item| item.to_uppercase()).collect::<Vec<String>>().join(\"\");"] # [doc = " println!(\"{}\", output);"] # [doc = " ```"] # [doc = " The correct use would be:"] # [doc = " ```no_run"] # [doc = " let vector = vec![\"hello\",  \"world\"];"] # [doc = " let output = vector.iter().map(|item| item.to_uppercase()).collect::<String>();"] # [doc = " println!(\"{}\", output);"] # [doc = " ```"] # [doc = " ### Known problems"] # [doc = " While `.collect::<String>()` is sometimes more performant, there are cases where"] # [doc = " using `.collect::<String>()` over `.collect::<Vec<String>>().join(\"\")`"] # [doc = " will prevent loop unrolling and will result in a negative performance impact."] # [doc = ""] # [doc = " Additionally, differences have been observed between aarch64 and x86_64 assembly output,"] # [doc = " with aarch64 tending to producing faster assembly in more cases when using `.collect::<String>()`"] # [clippy :: version = "1.61.0"] pub UNNECESSARY_JOIN , pedantic , "using `.collect::<Vec<String>>().join(\"\")` on an iterator" }
};
}
