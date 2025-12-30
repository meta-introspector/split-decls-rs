// Generated macro for TrplFigure (struct)
macro_rules! Depcrate_figureTrplFigure {
() => {
// Module: crate::figure
// Provides: {"TrplFigure"}
// Dependencies: {}
# [doc = " A simple preprocessor to rewrite `<figure>`s with `<img>`s."] # [doc = ""] # [doc = " This is a no-op by default; it only operates on the book chapters when the"] # [doc = " `[preprocessor.trpl-figure]` has `output-mode = \"simple\"`."] # [doc = ""] # [doc = " Takes in Markdown containing like this:"] # [doc = ""] # [doc = " ```markdown"] # [doc = " <figure>"] # [doc = ""] # [doc = " <img src=\"http://www.example.com/some-cool-image.jpg\">"] # [doc = ""] # [doc = " <figcaption>Figure 1-2: A description of the image</figcaption>"] # [doc = ""] # [doc = " </figure>"] # [doc = " ```"] # [doc = ""] # [doc = " Spits out Markdown like this:"] # [doc = ""] # [doc = " ```markdown"] # [doc = ""] # [doc = " <img src=\"http://www.example.com/some-cool-image.jpg\">"] # [doc = ""] # [doc = " Figure 1-2: A description of the image"] # [doc = ""] # [doc = " ```"] pub struct TrplFigure ;
};
}
