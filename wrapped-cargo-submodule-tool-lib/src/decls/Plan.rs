macro_rules! deps {
    () => {
        Task!();
    };
}

macro_rules! Plan {
    () => {
        deps!();
        # [cfg_attr (feature = "serde_enabled" , derive (Debug , Deserialize , Serialize))] pub struct Plan { pub tasks : HashMap < String , Task > , }
    };
}

Plan!()