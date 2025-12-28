macro_rules! SplitRule {
    () => {
        # [derive (Deserialize)] pub struct SplitRule { pub pattern : String , pub wrap_with : Vec < String > , pub imports : Vec < String > , }
    };
}

SplitRule!();