macro_rules! deps {
    () => {
        Error!();
        Value!();
        Result!();
    };
}

macro_rules! from_value {
    () => {
        deps!();
        # [doc = " Interpret a `serde_json::Value` as an instance of type `T`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Deserialize;"] # [doc = " use serde_json::json;"] # [doc = ""] # [doc = " #[derive(Deserialize, Debug)]"] # [doc = " struct User {"] # [doc = "     fingerprint: String,"] # [doc = "     location: String,"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     // The type of `j` is `serde_json::Value`"] # [doc = "     let j = json!({"] # [doc = "         \"fingerprint\": \"0xF9BA143B95FF6D82\","] # [doc = "         \"location\": \"Menlo Park, CA\""] # [doc = "     });"] # [doc = ""] # [doc = "     let u: User = serde_json::from_value(j).unwrap();"] # [doc = "     println!(\"{:#?}\", u);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This conversion can fail if the structure of the Value does not match the"] # [doc = " structure expected by `T`, for example if `T` is a struct type but the Value"] # [doc = " contains something other than a JSON map. It can also fail if the structure"] # [doc = " is correct but `T`'s implementation of `Deserialize` decides that something"] # [doc = " is wrong with the data, for example required struct fields are missing from"] # [doc = " the JSON map or some number is too big to fit in the expected primitive"] # [doc = " type."] pub fn from_value < T > (value : Value) -> Result < T , Error > where T : DeserializeOwned , { T :: deserialize (value) }
    };
}

from_value!()