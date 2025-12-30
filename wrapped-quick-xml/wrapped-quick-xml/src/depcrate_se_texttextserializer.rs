// Generated macro for TextSerializer (struct)
macro_rules! Depcrate_se_textTextSerializer {
() => {
// Module: crate::se::text
// Provides: {"TextSerializer"}
// Dependencies: {}
# [doc = " A serializer used to serialize a `$text` field of a struct or map."] # [doc = ""] # [doc = " This serializer a very similar to [`SimpleTypeSerializer`], but different"] # [doc = " from it in how it processes unit enum variants. Unlike [`SimpleTypeSerializer`]"] # [doc = " this serializer does not write anything for the unit variant."] pub struct TextSerializer < W : Write > (pub SimpleTypeSerializer < W >) ;
};
}
