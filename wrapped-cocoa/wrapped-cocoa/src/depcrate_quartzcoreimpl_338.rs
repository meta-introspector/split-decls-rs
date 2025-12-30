// Generated macro for impl_338 (impl)
macro_rules! Depcrate_quartzcoreimpl_338 {
() => {
// Module: crate::quartzcore
// Provides: {"impl_338"}
// Dependencies: {}
impl ContentsFormat { fn into_CFString (self) -> CFString { let string = match self { ContentsFormat :: RGBA8Uint => "RGBA8" , ContentsFormat :: RGBA16Float => "RGBAh" , ContentsFormat :: Gray8Uint => "Gray8" , ContentsFormat :: Other (other) => return other , } ; CFString :: from (string) } fn from_CFString (string : CFString) -> ContentsFormat { match string . to_string () { ref s if s == "RGBA8" => ContentsFormat :: RGBA8Uint , ref s if s == "RGBAh" => ContentsFormat :: RGBA16Float , ref s if s == "Gray8" => ContentsFormat :: Gray8Uint , _ => ContentsFormat :: Other (string) , } } }
};
}
