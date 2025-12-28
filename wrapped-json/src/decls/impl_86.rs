macro_rules! deps {
    () => {
        Map!();
        Value!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        # [doc = " Mutably access an element of this map. Panics if the given key is not"] # [doc = " present in the map."] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_json::json;"] # [doc = " #"] # [doc = " # let mut map = serde_json::Map::new();"] # [doc = " # map.insert(\"key\".to_owned(), serde_json::Value::Null);"] # [doc = " #"] # [doc = " map[\"key\"] = json!(\"value\");"] # [doc = " ```"] impl < Q > ops :: IndexMut < & Q > for Map < String , Value > where String : Borrow < Q > , Q : ? Sized + Ord + Eq + Hash , { fn index_mut (& mut self , index : & Q) -> & mut Value { self . map . get_mut (index) . expect ("no entry found for key") } }
    };
}

impl_86!();