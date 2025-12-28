macro_rules! deps {
    () => {
        Result!();
        SliceRead!();
    };
}

macro_rules! from_slice {
    () => {
        deps!();
        # [doc = " Deserialize an instance of type `T` from bytes of JSON text."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Deserialize;"] # [doc = ""] # [doc = " #[derive(Deserialize, Debug)]"] # [doc = " struct User {"] # [doc = "     fingerprint: String,"] # [doc = "     location: String,"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     // The type of `j` is `&[u8]`"] # [doc = "     let j = b\""] # [doc = "         {"] # [doc = "             \\\"fingerprint\\\": \\\"0xF9BA143B95FF6D82\\\","] # [doc = "             \\\"location\\\": \\\"Menlo Park, CA\\\""] # [doc = "         }\";"] # [doc = ""] # [doc = "     let u: User = serde_json::from_slice(j).unwrap();"] # [doc = "     println!(\"{:#?}\", u);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This conversion can fail if the structure of the input does not match the"] # [doc = " structure expected by `T`, for example if `T` is a struct type but the input"] # [doc = " contains something other than a JSON map. It can also fail if the structure"] # [doc = " is correct but `T`'s implementation of `Deserialize` decides that something"] # [doc = " is wrong with the data, for example required struct fields are missing from"] # [doc = " the JSON map or some number is too big to fit in the expected primitive"] # [doc = " type."] pub fn from_slice < 'a , T > (v : & 'a [u8]) -> Result < T > where T : de :: Deserialize < 'a > , { from_trait (read :: SliceRead :: new (v)) }
    };
}

from_slice!();