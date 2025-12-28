macro_rules! deps {
    () => {
        Value!();
        Result!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        # [doc = " The default value is `Value::Null`."] # [doc = ""] # [doc = " This is useful for handling omitted `Value` fields when deserializing."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use serde::Deserialize;"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct Settings {"] # [doc = "     level: i32,"] # [doc = "     #[serde(default)]"] # [doc = "     extras: Value,"] # [doc = " }"] # [doc = ""] # [doc = " # fn try_main() -> Result<(), serde_json::Error> {"] # [doc = " let data = r#\" { \"level\": 42 } \"#;"] # [doc = " let s: Settings = serde_json::from_str(data)?;"] # [doc = ""] # [doc = " assert_eq!(s.level, 42);"] # [doc = " assert_eq!(s.extras, Value::Null);"] # [doc = " #"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " #"] # [doc = " # try_main().unwrap()"] # [doc = " ```"] impl Default for Value { fn default () -> Value { Value :: Null } }
    };
}

impl_246!();