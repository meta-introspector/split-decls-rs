// Generated macro for impl_227 (impl)
macro_rules! Depcrate_responseimpl_227 {
() => {
// Module: crate::response
// Provides: {"impl_227"}
// Dependencies: {}
# [doc = " Cloning a DataPayload is generally a cheap operation."] # [doc = " See notes in the `Clone` impl for [`Yoke`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use icu_provider::hello_world::*;"] # [doc = " use icu_provider::prelude::*;"] # [doc = ""] # [doc = " let resp1: DataPayload<HelloWorldV1> = todo!();"] # [doc = " let resp2 = resp1.clone();"] # [doc = " ```"] impl < M > Clone for DataPayload < M > where M : DynamicDataMarker , for < 'a > < M :: DataStruct as Yokeable < 'a > > :: Output : Clone , { fn clone (& self) -> Self { Self (match & self . 0 { DataPayloadInner :: Yoke (yoke) => DataPayloadInner :: Yoke (yoke . clone ()) , DataPayloadInner :: StaticRef (r) => DataPayloadInner :: StaticRef (* r) , }) } }
};
}
