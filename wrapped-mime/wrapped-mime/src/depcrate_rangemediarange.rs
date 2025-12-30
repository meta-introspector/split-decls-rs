// Generated macro for MediaRange (struct)
macro_rules! Depcrate_rangeMediaRange {
() => {
// Module: crate::range
// Provides: {"MediaRange"}
// Dependencies: {}
# [doc = " A parsed media range used to match media types."] # [doc = ""] # [doc = " Commonly found in the HTTP `Accept` header, these are used for agents"] # [doc = " to indicate general classes of content that they can understand."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use mime::MediaRange;"] # [doc = ""] # [doc = " // Suppose this was parsed from an `Accept` header"] # [doc = " let range = MediaRange::parse(\"text/*\").unwrap();"] # [doc = ""] # [doc = " // The formats of the resource that we have:"] # [doc = " let formats = vec!["] # [doc = "     mime::APPLICATION_JSON,"] # [doc = "     mime::TEXT_PLAIN_UTF_8,"] # [doc = "     mime::TEXT_HTML,"] # [doc = " ];"] # [doc = ""] # [doc = " for format in &formats {"] # [doc = "     if range.matches(format) {"] # [doc = "         // This should print for the plain and HTML text..."] # [doc = "         println!(\"We could send in {:?} format!\", format);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [derive (Clone)] pub struct MediaRange { pub (super) mime : Mime , }
};
}
