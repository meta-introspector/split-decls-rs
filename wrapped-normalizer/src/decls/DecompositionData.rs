macro_rules! deps {
    () => {
        Decomposition!();
        Trie!();
    };
}

macro_rules! DecompositionData {
    () => {
        deps!();
        # [doc = " Decomposition data"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_normalizer :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct DecompositionData < 'data > { # [doc = " Trie for decomposition."] # [cfg_attr (feature = "serde" , serde (borrow))] pub trie : CodePointTrie < 'data , u32 > , # [doc = " The passthrough bounds of NFD/NFC are lowered to this"] # [doc = " maximum instead. (16-bit, because cannot be higher"] # [doc = " than 0x0300, which is the bound for NFC.)"] pub passthrough_cap : u16 , }
    };
}

DecompositionData!();