macro_rules! deps {
    () => {
        Map!();
        Value!();
        Index!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        # [doc = " Access an element of this map. Panics if the given key is not present in the"] # [doc = " map."] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_json::Value;"] # [doc = " #"] # [doc = " # let val = &Value::String(\"\".to_owned());"] # [doc = " # let _ ="] # [doc = " match val {"] # [doc = "     Value::String(s) => Some(s.as_str()),"] # [doc = "     Value::Array(arr) => arr[0].as_str(),"] # [doc = "     Value::Object(map) => map[\"type\"].as_str(),"] # [doc = "     _ => None,"] # [doc = " }"] # [doc = " # ;"] # [doc = " ```"] impl < Q > ops :: Index < & Q > for Map < String , Value > where String : Borrow < Q > , Q : ? Sized + Ord + Eq + Hash , { type Output = Value ; fn index (& self , index : & Q) -> & Value { self . map . index (index) } }
    };
}

impl_85!();