macro_rules! Either {
    () => {
        # [doc = " The enum `Either` with variants `Left` and `Right` is a general purpose"] # [doc = " sum type with two cases."] # [doc = ""] # [doc = " The `Either` type is symmetric and treats its variants the same way, without"] # [doc = " preference."] # [doc = " (For representing success or error, use the regular `Result` enum instead.)"] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [derive (Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] pub enum Either < L , R > { # [doc = " A value of type `L`."] Left (L) , # [doc = " A value of type `R`."] Right (R) , }
    };
}

Either!()