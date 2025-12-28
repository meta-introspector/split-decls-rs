macro_rules! deps {
    () => {
        EnumeratedProperty!();
        BidiMirroringGlyph!();
    };
}

macro_rules! BidiPairedBracketType {
    () => {
        deps!();
        # [doc = " The enum represents Bidi_Paired_Bracket_Type."] # [doc = ""] # [doc = " It does not implement [`EnumeratedProperty`], instead it can be obtained"] # [doc = " through the bitpacked [`BidiMirroringGlyph`] property."] # [doc = ""] # [doc = " If you have a use case this property without also needing the [`BidiMirroringGlyph`]"] # [doc = " property, and need to optimize data size, please file an issue."] # [derive (Debug , Eq , PartialEq , Copy , Clone , Default)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_properties :: props))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [non_exhaustive] pub enum BidiPairedBracketType { # [doc = " Represents Bidi_Paired_Bracket_Type=Open."] Open , # [doc = " Represents Bidi_Paired_Bracket_Type=Close."] Close , # [doc = " Represents Bidi_Paired_Bracket_Type=None."] # [default] None , }
    };
}

BidiPairedBracketType!();