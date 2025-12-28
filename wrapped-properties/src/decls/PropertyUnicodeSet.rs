macro_rules! PropertyUnicodeSet {
    () => {
        # [doc = " A set of characters and strings which share a particular property value."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , Eq , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_properties :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [non_exhaustive] pub enum PropertyUnicodeSet < 'data > { # [doc = " A set representing characters in an inversion list, and the strings in a list."] CPInversionListStrList (# [cfg_attr (feature = "serde" , serde (borrow))] CodePointInversionListAndStringList < 'data > ,) , }
    };
}

PropertyUnicodeSet!();