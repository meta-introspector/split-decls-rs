macro_rules! EditionData {
    () => {
        # [derive (Serialize , Deserialize , Debug , Clone , Eq , PartialEq)] # [serde (rename = "edition")] enum EditionData { # [serde (rename = "2015")] Edition2015 , # [serde (rename = "2018")] Edition2018 , # [serde (rename = "2021")] Edition2021 , # [serde (rename = "2024")] Edition2024 , }
    };
}

EditionData!()