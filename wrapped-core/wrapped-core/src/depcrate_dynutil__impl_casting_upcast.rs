// Generated macro for __impl_casting_upcast (macro)
macro_rules! Depcrate_dynutil__impl_casting_upcast {
() => {
// Module: crate::dynutil
// Provides: {"__impl_casting_upcast"}
// Dependencies: {}
# [doc = " Implements [`UpcastDataPayload`] from several data markers to a single data marker"] # [doc = " that all share the same [`DynamicDataMarker::DataStruct`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu_provider::prelude::*;"] # [doc = " use std::borrow::Cow;"] # [doc = ""] # [doc = " struct FooV1;"] # [doc = " impl DynamicDataMarker for FooV1 {"] # [doc = "     type DataStruct = Foo<'static>;"] # [doc = " }"] # [doc = " icu_provider::data_marker!(BarV1, Foo<'static>);"] # [doc = " icu_provider::data_marker!(BazV1, Foo<'static>);"] # [doc = ""] # [doc = " #[derive(yoke::Yokeable)]"] # [doc = " pub struct Foo<'data> {"] # [doc = "     message: Cow<'data, str>,"] # [doc = " };"] # [doc = ""] # [doc = " icu_provider::data_struct!(Foo<'_>);"] # [doc = ""] # [doc = " icu_provider::dynutil::impl_casting_upcast!(FooV1, [BarV1, BazV1,]);"] # [doc = " ```"] # [doc = ""] # [doc = " [`DynamicDataMarker::DataStruct`]: crate::DynamicDataMarker::DataStruct"] # [macro_export] # [doc (hidden)] macro_rules ! __impl_casting_upcast { ($ dyn_m : path , [$ ($ struct_m : ident) ,+,]) => { $ (impl $ crate :: dynutil :: UpcastDataPayload <$ struct_m > for $ dyn_m { fn upcast (other : $ crate :: DataPayload <$ struct_m >) -> $ crate :: DataPayload <$ dyn_m > { other . cast () } }) + } }
};
}
