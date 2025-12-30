// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Lint { fn doc_contains (& self , text : & str) -> bool { self . doc . iter () . any (| line | line . contains (text)) } fn is_ignored (& self) -> bool { let blocks : Vec < _ > = self . doc . iter () . filter (| line | line . starts_with ("```rust")) . collect () ; ! blocks . is_empty () && blocks . iter () . all (| line | line . contains (",ignore")) } # [doc = " Checks the doc style of the lint."] fn check_style (& self) -> Result < () , Box < dyn Error > > { for & expected in & ["### Example" , "### Explanation" , "{{produces}}"] { if expected == "{{produces}}" && self . is_ignored () { if self . doc_contains ("{{produces}}") { return Err ("the lint example has `ignore`, but also contains the {{produces}} marker\n\
                        \n\
                        The documentation generator cannot generate the example output when the \
                        example is ignored.\n\
                        Manually include the sample output below the example. For example:\n\
                        \n\
                        /// ```rust,ignore (needs command line option)\n\
                        /// #[cfg(widnows)]\n\
                        /// fn foo() {{}}\n\
                        /// ```\n\
                        ///\n\
                        /// This will produce:\n\
                        /// \n\
                        /// ```text\n\
                        /// warning: unknown condition name used\n\
                        ///  --> lint_example.rs:1:7\n\
                        ///   |\n\
                        /// 1 | #[cfg(widnows)]\n\
                        ///   |       ^^^^^^^\n\
                        ///   |\n\
                        ///   = note: `#[warn(unexpected_cfgs)]` on by default\n\
                        /// ```\n\
                        \n\
                        Replacing the output with the text of the example you \
                        compiled manually yourself.\n\
                        " . into ()) ; } continue ; } if ! self . doc_contains (expected) { return Err (format ! ("lint docs should contain the line `{}`" , expected) . into ()) ; } } if let Some (first) = self . doc . first () { if ! first . starts_with (& format ! ("The `{}` lint" , self . name)) { return Err (format ! ("lint docs should start with the text \"The `{}` lint\" to introduce the lint" , self . name) . into ()) ; } } Ok (()) } }
};
}
