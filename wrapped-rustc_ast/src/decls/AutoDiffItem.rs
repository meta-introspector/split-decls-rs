macro_rules! deps {
    () => {
        AutoDiffAttrs!();
    };
}

macro_rules! AutoDiffItem {
    () => {
        deps!();
        # [doc = " We generate one of these structs for each `#[autodiff(...)]` attribute."] # [derive (Clone , Eq , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct AutoDiffItem { # [doc = " The name of the function getting differentiated"] pub source : String , # [doc = " The name of the function being generated"] pub target : String , pub attrs : AutoDiffAttrs , }
    };
}

AutoDiffItem!();