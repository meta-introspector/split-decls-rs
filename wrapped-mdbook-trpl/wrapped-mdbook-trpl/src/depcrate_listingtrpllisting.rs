// Generated macro for TrplListing (struct)
macro_rules! Depcrate_listingTrplListing {
() => {
// Module: crate::listing
// Provides: {"TrplListing"}
// Dependencies: {}
# [doc = " A preprocessor for rendering listings more elegantly."] # [doc = ""] # [doc = " Given input like this:"] # [doc = ""] # [doc = " ````markdown"] # [doc = " <Listing number=\"1-2\" file-name=\"src/main.rs\" caption=\"Some *text*, yeah?\">"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " </Listing>"] # [doc = ""] # [doc = " ````"] # [doc = ""] # [doc = " With no configuration, or with `output-mode = \"default\"`, it renders the"] # [doc = " following Markdown to be further preprocessed or rendered to HTML:"] # [doc = ""] # [doc = " ````markdown"] # [doc = " <figure class=\"listing\">"] # [doc = " <span class=\"file-name\">Filename: src/main.rs</span>"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " <figcaption>Listing 1-2: Some <em>text</em>, yeah?</figcaption>"] # [doc = ""] # [doc = " </figure>"] # [doc = " ````"] # [doc = ""] # [doc = " When `output-mode = \"simple\"` in the configuration, it instead emits:"] # [doc = ""] # [doc = " ````markdown"] # [doc = " Filename: src/main.rs"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " Listing 1-2: Some *text*, yeah?"] # [doc = " ````"] pub struct TrplListing ;
};
}
