// Generated macro for HelloWorldJsonProvider (struct)
macro_rules! Depcrate_hello_worldHelloWorldJsonProvider {
() => {
// Module: crate::hello_world
// Provides: {"HelloWorldJsonProvider"}
// Dependencies: {}
# [cfg (feature = "deserialize_json")] # [doc = " A data provider returning Hello World strings in different languages as JSON blobs."] # [doc = ""] # [doc = " Mostly useful for testing."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu_locale_core::langid;"] # [doc = " use icu_provider::hello_world::*;"] # [doc = " use icu_provider::prelude::*;"] # [doc = ""] # [doc = " let german_hello_world = HelloWorldProvider"] # [doc = "     .into_json_provider()"] # [doc = "     .load_data(HelloWorldV1::INFO, DataRequest {"] # [doc = "         id: DataIdentifierBorrowed::for_locale(&langid!(\"de\").into()),"] # [doc = "         ..Default::default()"] # [doc = "     })"] # [doc = "     .expect(\"Loading should succeed\");"] # [doc = ""] # [doc = " assert_eq!(german_hello_world.payload.get(), br#\"{\"message\":\"Hallo Welt\"}\"#);"] # [derive (Debug)] pub struct HelloWorldJsonProvider ;
};
}
