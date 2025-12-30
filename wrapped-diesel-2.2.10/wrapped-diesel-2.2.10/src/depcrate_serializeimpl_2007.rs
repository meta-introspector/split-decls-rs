// Generated macro for impl_2007 (impl)
macro_rules! Depcrate_serializeimpl_2007 {
() => {
// Module: crate::serialize
// Provides: {"impl_2007"}
// Dependencies: {}
impl < 'a , DB > Output < 'a , '_ , DB > where for < 'c > DB : Backend < BindCollector < 'c > = RawBytesBindCollector < DB > > , { # [doc = " Call this method whenever you pass an instance of `Output<DB>` by value."] # [doc = ""] # [doc = " Effectively copies `self`, with a narrower lifetime. When passing a"] # [doc = " reference or a mutable reference, this is normally done by rust"] # [doc = " implicitly. This is why you can pass `&mut Foo` to multiple functions,"] # [doc = " even though mutable references are not `Copy`. However, this is only"] # [doc = " done implicitly for references. For structs with lifetimes it must be"] # [doc = " done explicitly. This method matches the semantics of what Rust would do"] # [doc = " implicitly if you were passing a mutable reference"] pub fn reborrow < 'c > (& 'c mut self) -> Output < 'c , 'c , DB > where 'a : 'c , { Output { out : RawBytesBindCollector :: < DB > :: reborrow_buffer (& mut self . out) , metadata_lookup : match & mut self . metadata_lookup { None => None , Some (m) => Some (& mut * * m) , } , } } }
};
}
