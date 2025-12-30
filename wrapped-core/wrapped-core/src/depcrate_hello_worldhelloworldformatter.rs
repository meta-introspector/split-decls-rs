// Generated macro for HelloWorldFormatter (struct)
macro_rules! Depcrate_hello_worldHelloWorldFormatter {
() => {
// Module: crate::hello_world
// Provides: {"HelloWorldFormatter"}
// Dependencies: {}
# [doc = " A type that formats localized \"hello world\" strings."] # [doc = ""] # [doc = " This type is intended to take the shape of a typical ICU4X formatter API."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu_locale_core::locale;"] # [doc = " use icu_provider::hello_world::{HelloWorldFormatter, HelloWorldProvider};"] # [doc = " use writeable::assert_writeable_eq;"] # [doc = ""] # [doc = " let fmt = HelloWorldFormatter::try_new_unstable("] # [doc = "     &HelloWorldProvider,"] # [doc = "     locale!(\"eo\").into(),"] # [doc = " )"] # [doc = " .expect(\"locale exists\");"] # [doc = ""] # [doc = " assert_writeable_eq!(fmt.format(), \"Saluton, Mondo\");"] # [doc = " ```"] # [derive (Debug)] pub struct HelloWorldFormatter { data : DataPayload < HelloWorldV1 > , }
};
}
