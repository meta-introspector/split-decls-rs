// Generated macro for macro_1779 (macro)
macro_rules! Depcrate_docmacro_1779 {
() => {
// Module: crate::doc
// Provides: {"macro_1779"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects syntax that looks like a footnote reference."] # [doc = ""] # [doc = " Rustdoc footnotes are compatible with GitHub-Flavored Markdown (GFM)."] # [doc = " GFM does not parse a footnote reference unless its definition also"] # [doc = " exists. This lint checks for footnote references with missing"] # [doc = " definitions, unless it thinks you're writing a regex."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This probably means that a footnote was meant to exist,"] # [doc = " but was not written."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " /// This is not a footnote[^1], because no definition exists."] # [doc = " fn my_fn() {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " /// This is a footnote[^1]."] # [doc = " ///"] # [doc = " /// [^1]: defined here"] # [doc = " fn my_fn() {}"] # [doc = " ```"] # [clippy :: version = "1.89.0"] pub DOC_SUSPICIOUS_FOOTNOTES , suspicious , "looks like a link or footnote ref, but with no definition" }
};
}
