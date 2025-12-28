macro_rules! Edition {
    () => {
        # [doc = " The Rust edition"] # [doc = ""] # [doc = " As of writing this comment rust editions 2027 and 2030 are not actually a thing yet but are parsed nonetheless for future proofing."] # [derive (Debug , Clone , Copy , Serialize , Deserialize , PartialEq , Eq , Hash , PartialOrd , Ord)] # [non_exhaustive] # [derive (Default)] pub enum Edition { # [doc = " Edition 2015"] # [serde (rename = "2015")] # [default] E2015 , # [doc = " Edition 2018"] # [serde (rename = "2018")] E2018 , # [doc = " Edition 2021"] # [serde (rename = "2021")] E2021 , # [doc = " Edition 2024"] # [serde (rename = "2024")] E2024 , # [doc (hidden)] # [serde (rename = "2027")] _E2027 , # [doc (hidden)] # [serde (rename = "2030")] _E2030 , }
    };
}

Edition!()