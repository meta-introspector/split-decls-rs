// Generated macro for impl_67 (impl)
macro_rules! Depcrate_header_mapimpl_67 {
() => {
// Module: crate::header::map
// Provides: {"impl_67"}
// Dependencies: {}
impl < K , T > ops :: Index < K > for HeaderMap < T > where K : AsHeaderName , { type Output = T ; # [doc = " # Panics"] # [doc = " Using the index operator will cause a panic if the header you're querying isn't set."] # [inline] fn index (& self , index : K) -> & T { match self . get2 (& index) { Some (val) => val , None => panic ! ("no entry found for key {:?}" , index . as_str ()) , } } }
};
}
