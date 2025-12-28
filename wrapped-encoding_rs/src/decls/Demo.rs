macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! Demo {
    () => {
        deps!();
        # [cfg (all (test , feature = "serde"))] # [derive (Serialize , Deserialize , Debug , PartialEq)] struct Demo { num : u32 , name : String , enc : & 'static Encoding , }
    };
}

Demo!()