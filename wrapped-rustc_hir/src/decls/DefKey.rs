macro_rules! deps {
    () => {
        DisambiguatedDefPathData!();
    };
}

macro_rules! DefKey {
    () => {
        deps!();
        # [doc = " A unique identifier that we can use to lookup a definition"] # [doc = " precisely. It combines the index of the definition's parent (if"] # [doc = " any) with a `DisambiguatedDefPathData`."] # [derive (Copy , Clone , PartialEq , Debug , Encodable , Decodable)] pub struct DefKey { # [doc = " The parent path."] pub parent : Option < DefIndex > , # [doc = " The identifier of this node."] pub disambiguated_data : DisambiguatedDefPathData , }
    };
}

DefKey!();