macro_rules! GemEntry {
    () => {
        # [derive (Debug , Deserialize)] pub struct GemEntry { pub name : String , pub crate_name : String , pub identifiers : Vec < String > , }
    };
}

GemEntry!()