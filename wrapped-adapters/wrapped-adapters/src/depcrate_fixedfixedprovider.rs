// Generated macro for FixedProvider (struct)
macro_rules! Depcrate_fixedFixedProvider {
() => {
// Module: crate::fixed
// Provides: {"FixedProvider"}
// Dependencies: {}
# [doc = " A data provider that returns clones of a fixed type-erased payload."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu_provider::hello_world::*;"] # [doc = " use icu_provider::prelude::*;"] # [doc = " use icu_provider_adapters::fixed::FixedProvider;"] # [doc = " use std::borrow::Cow;"] # [doc = " use writeable::assert_writeable_eq;"] # [doc = ""] # [doc = " let provider = FixedProvider::<HelloWorldV1>::from_static(&HelloWorld {"] # [doc = "     message: Cow::Borrowed(\"custom hello world\"),"] # [doc = " });"] # [doc = ""] # [doc = " // Check that it works:"] # [doc = " let formatter ="] # [doc = "     HelloWorldFormatter::try_new_unstable(&provider, Default::default())"] # [doc = "         .expect(\"marker matches\");"] # [doc = " assert_writeable_eq!(formatter.format(), \"custom hello world\");"] # [doc = " ```"] # [allow (clippy :: exhaustive_structs)] pub struct FixedProvider < M : DataMarker > { data : DataPayload < M > , }
};
}
