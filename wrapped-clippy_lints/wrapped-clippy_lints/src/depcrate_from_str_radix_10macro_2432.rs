// Generated macro for macro_2432 (macro)
macro_rules! Depcrate_from_str_radix_10macro_2432 {
() => {
// Module: crate::from_str_radix_10
// Provides: {"macro_2432"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for function invocations of the form `primitive::from_str_radix(s, 10)`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " This specific common use case can be rewritten as `s.parse::<primitive>()`"] # [doc = " (and in most cases, the turbofish can be removed), which reduces code length"] # [doc = " and complexity."] # [doc = ""] # [doc = " ### Known problems"] # [doc = ""] # [doc = " This lint may suggest using `(&<expression>).parse()` instead of `<expression>.parse()`"] # [doc = " directly in some cases, which is correct but adds unnecessary complexity to the code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " let input: &str = get_input();"] # [doc = " let num = u16::from_str_radix(input, 10)?;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " let input: &str = get_input();"] # [doc = " let num: u16 = input.parse()?;"] # [doc = " ```"] # [clippy :: version = "1.52.0"] pub FROM_STR_RADIX_10 , style , "from_str_radix with radix 10" }
};
}
