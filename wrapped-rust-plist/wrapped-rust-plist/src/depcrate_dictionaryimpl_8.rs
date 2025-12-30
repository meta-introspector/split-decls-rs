// Generated macro for impl_8 (impl)
macro_rules! Depcrate_dictionaryimpl_8 {
() => {
// Module: crate::dictionary
// Provides: {"impl_8"}
// Dependencies: {}
# [doc = " Mutably access an element of this dictionary. Panics if the given key is not present in the"] # [doc = " dictionary."] # [doc = ""] # [doc = " ```"] # [doc = " # let mut dict = plist::Dictionary::new();"] # [doc = " # dict.insert(\"key\".to_owned(), plist::Value::Boolean(false));"] # [doc = " #"] # [doc = " dict[\"key\"] = \"value\".into();"] # [doc = " ```"] impl ops :: IndexMut < & str > for Dictionary { fn index_mut (& mut self , index : & str) -> & mut Value { self . map . get_mut (index) . expect ("no entry found for key") } }
};
}
