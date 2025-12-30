// Generated macro for impl_93 (impl)
macro_rules! Depcrate_header_mapimpl_93 {
() => {
// Module: crate::header::map
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'a , T : 'a > GetAll < 'a , T > { # [doc = " Returns an iterator visiting all values associated with the entry."] # [doc = ""] # [doc = " Values are iterated in insertion order."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use http::HeaderMap;"] # [doc = " # use http::header::HOST;"] # [doc = " let mut map = HeaderMap::new();"] # [doc = " map.insert(HOST, \"hello.world\".parse().unwrap());"] # [doc = " map.append(HOST, \"hello.earth\".parse().unwrap());"] # [doc = ""] # [doc = " let values = map.get_all(\"host\");"] # [doc = " let mut iter = values.iter();"] # [doc = " assert_eq!(&\"hello.world\", iter.next().unwrap());"] # [doc = " assert_eq!(&\"hello.earth\", iter.next().unwrap());"] # [doc = " assert!(iter.next().is_none());"] # [doc = " ```"] pub fn iter (& self) -> ValueIter < 'a , T > { GetAll { map : self . map , index : self . index , } . into_iter () } }
};
}
