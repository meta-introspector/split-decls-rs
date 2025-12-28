macro_rules! deps {
    () => {
        Result!();
        Error!();
        RawValue!();
    };
}

macro_rules! impl_612 {
    () => {
        deps!();
        impl RawValue { # [doc = " A constant RawValue with the JSON value `null`."] pub const NULL : & 'static RawValue = RawValue :: from_borrowed ("null") ; # [doc = " A constant RawValue with the JSON value `true`."] pub const TRUE : & 'static RawValue = RawValue :: from_borrowed ("true") ; # [doc = " A constant RawValue with the JSON value `false`."] pub const FALSE : & 'static RawValue = RawValue :: from_borrowed ("false") ; # [doc = " Convert an owned `String` of JSON data to an owned `RawValue`."] # [doc = ""] # [doc = " This function is equivalent to `serde_json::from_str::<Box<RawValue>>`"] # [doc = " except that we avoid an allocation and memcpy if both of the following"] # [doc = " are true:"] # [doc = ""] # [doc = " - the input has no leading or trailing whitespace, and"] # [doc = " - the input has capacity equal to its length."] pub fn from_string (json : String) -> Result < Box < Self > , Error > { let borrowed = tri ! (crate :: from_str ::<& Self > (& json)) ; if borrowed . json . len () < json . len () { return Ok (borrowed . to_owned ()) ; } Ok (Self :: from_owned (json . into_boxed_str ())) } # [doc = " Access the JSON text underlying a raw value."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Deserialize;"] # [doc = " use serde_json::{Result, value::RawValue};"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Response<'a> {"] # [doc = "     code: u32,"] # [doc = "     #[serde(borrow)]"] # [doc = "     payload: &'a RawValue,"] # [doc = " }"] # [doc = ""] # [doc = " fn process(input: &str) -> Result<()> {"] # [doc = "     let response: Response = serde_json::from_str(input)?;"] # [doc = ""] # [doc = "     let payload = response.payload.get();"] # [doc = "     if payload.starts_with('{') {"] # [doc = "         // handle a payload which is a JSON map"] # [doc = "     } else {"] # [doc = "         // handle any other type"] # [doc = "     }"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = ""] # [doc = " fn main() -> Result<()> {"] # [doc = "     process(r#\" {\"code\": 200, \"payload\": {}} \"#)?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] pub fn get (& self) -> & str { & self . json } }
    };
}

impl_612!()