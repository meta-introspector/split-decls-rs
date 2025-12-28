macro_rules! ID {
    () => {
        # [doc = " ID scalar"] # [doc = ""] # [doc = " The input is a `&str`, `String`, `usize` or `uuid::UUID`, and the output is"] # [doc = " a string."] # [derive (Clone , Ord , PartialOrd , Eq , PartialEq , Hash , Debug , Serialize , Deserialize , Default)] # [serde (transparent)] pub struct ID (pub String) ;
    };
}

ID!()