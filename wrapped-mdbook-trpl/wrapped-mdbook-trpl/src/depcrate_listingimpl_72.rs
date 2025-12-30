// Generated macro for impl_72 (impl)
macro_rules! Depcrate_listingimpl_72 {
() => {
// Module: crate::listing
// Provides: {"impl_72"}
// Dependencies: {}
impl Listing { fn opening_html (& self) -> String { let id_attribute = self . number . as_ref () . map (| number | format ! (" id=\"listing-{number}\"")) . unwrap_or_default () ; let figure = format ! ("<figure class=\"listing\"{id_attribute}>\n") ; match self . file_name . as_ref () { Some (file_name) => format ! ("{figure}<span class=\"file-name\">Filename: {file_name}</span>\n" ,) , None => figure , } } fn closing_html (& self , trailing : & str) -> String { match (& self . number , & self . caption) { (Some (number) , caption) => { let caption_text = caption . as_ref () . map (| caption | format ! (": {}" , caption)) . unwrap_or_default () ; let listing_a_tag = format ! ("<a href=\"#listing-{number}\">Listing {number}</a>") ; format ! (r#"<figcaption>{listing_a_tag}{caption_text}</figcaption>
</figure>{trailing}"#) } (None , Some (caption)) => format ! (r#"<figcaption>{caption}</figcaption>
</figure>{trailing}"#) , (None , None) => format ! ("</figure>{trailing}") , } } fn opening_text (& self) -> String { self . file_name . as_ref () . map (| file_name | format ! ("{file_name}\n")) . unwrap_or_default () } fn closing_text (& self , trailing : & str) -> String { match (& self . number , & self . caption) { (Some (number) , Some (caption)) => { format ! ("Listing {number}: {caption}{trailing}") } (None , Some (caption)) => format ! ("{caption}{trailing}") , (Some (number) , None) => format ! ("Listing {number}{trailing}") , (None , None) => trailing . into () , } } }
};
}
