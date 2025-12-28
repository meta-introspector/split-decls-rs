macro_rules! CanonicalCombiningClassMap {
    () => {
        # [doc = " Lookup of the Canonical_Combining_Class Unicode property."] # [derive (Debug)] pub struct CanonicalCombiningClassMap { # [doc = " The data trie"] decompositions : DataPayload < NormalizerNfdDataV1 > , }
    };
}

CanonicalCombiningClassMap!();