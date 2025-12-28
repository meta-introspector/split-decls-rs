macro_rules! deps {
    () => {
        BidiPairedBracketType!();
    };
}

macro_rules! BidiMirroringGlyph {
    () => {
        deps!();
        # [doc = " This is a bitpacked combination of the `Bidi_Mirroring_Glyph`,"] # [doc = " `Bidi_Mirrored`, and `Bidi_Paired_Bracket_Type` properties."] # [derive (Debug , Eq , PartialEq , Clone , Copy , Default)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_properties :: props))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] pub struct BidiMirroringGlyph { # [doc = " The mirroring glyph"] pub mirroring_glyph : Option < char > , # [doc = " Whether the glyph is mirrored"] pub mirrored : bool , # [doc = " The paired bracket type"] pub paired_bracket_type : BidiPairedBracketType , }
    };
}

BidiMirroringGlyph!();