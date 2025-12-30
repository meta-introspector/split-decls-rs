// Generated macro for impl_242 (impl)
macro_rules! Depcrate_responseimpl_242 {
() => {
// Module: crate::response
// Provides: {"impl_242"}
// Dependencies: {}
# [doc = " Cloning a DataResponse is generally a cheap operation."] # [doc = " See notes in the `Clone` impl for [`Yoke`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use icu_provider::hello_world::*;"] # [doc = " use icu_provider::prelude::*;"] # [doc = ""] # [doc = " let resp1: DataResponse<HelloWorldV1> = todo!();"] # [doc = " let resp2 = resp1.clone();"] # [doc = " ```"] impl < M > Clone for DataResponse < M > where M : DynamicDataMarker , for < 'a > < M :: DataStruct as Yokeable < 'a > > :: Output : Clone , { fn clone (& self) -> Self { Self { metadata : self . metadata . clone () , payload : self . payload . clone () , } } }
};
}
