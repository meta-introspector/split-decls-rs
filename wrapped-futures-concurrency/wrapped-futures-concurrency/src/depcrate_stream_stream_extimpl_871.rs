// Generated macro for impl_871 (impl)
macro_rules! Depcrate_stream_stream_extimpl_871 {
() => {
// Module: crate::stream::stream_ext
// Provides: {"impl_871"}
// Dependencies: {}
impl < S1 > StreamExt for S1 where S1 : Stream , { fn merge < T , S2 > (self , other : S2) -> Merge2 < T , S1 , S2 :: IntoStream > where S1 : Stream < Item = T > , S2 : IntoStream < Item = T > , { Merge :: merge ((self , other)) } fn chain < T , S2 > (self , other : S2) -> Chain2 < Self , S2 :: IntoStream > where Self : Stream < Item = T > + Sized , S2 : IntoStream < Item = T > , { Chain :: chain ((self , other . into_stream ())) } fn zip < T , S2 > (self , other : S2) -> Zip2 < Self , S2 :: IntoStream > where Self : Stream < Item = T > + Sized , S2 : IntoStream < Item = T > , { Zip :: zip ((self , other . into_stream ())) } }
};
}
