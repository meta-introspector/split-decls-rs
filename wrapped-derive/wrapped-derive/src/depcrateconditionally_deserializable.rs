// Generated macro for conditionally_deserializable (function)
macro_rules! Depcrateconditionally_deserializable {
() => {
// Module: crate
// Provides: {"conditionally_deserializable"}
// Dependencies: {}
# [doc = " The `conditionally_deserializable` attribute macro creates two versions of"] # [doc = " the affected struct: One that implements the"] # [doc = " [`Deserialize`](../tls_codec::Deserialize) and"] # [doc = " [`DeserializeBytes`](../tls_codec::DeserializeBytes) traits and one that"] # [doc = " does not. It does so by introducing a boolean const generic that indicates"] # [doc = " if the struct can be deserialized or not."] # [doc = ""] # [doc = " This conditional deserialization can be used, for example, to implement a"] # [doc = " simple state machine, where after deserialization, the user must first"] # [doc = " complete an additional transition (e.g. verification) to the"] # [doc = " undeserializable version of the struct, which might then implement functions"] # [doc = " for further processing."] # [doc = ""] # [doc = " For ease of use, the macro creates type aliases for the deserializable and"] # [doc = " undeserializable variant of the struct, where the alias is the name of the"] # [doc = " struct prefixed with `Deserializable` or `Undeserializable` respectively."] # [doc = ""] # [doc = " Due to the way that the macro rewrites the struct, it must be placed before"] # [doc = " any `#[derive(...)]` statements."] # [doc = ""] # [doc = " The `conditionally_deserializable` attribute macro is only available if the"] # [doc = " `conditional_deserialization` feature is enabled."] # [cfg_attr (feature = "conditional_deserialization" , doc = r##"
```compile_fail
use tls_codec_derive::{TlsSerialize, TlsDeserialize, TlsSize, conditionally_deserializable};

#[conditionally_deserializable(Bytes)]
#[derive(TlsDeserialize, TlsSerialize, TlsSize)]
struct ExampleStruct {
    pub a: u16,
}

impl UndeserializableExampleStruct {
    #[cfg(feature = "conditional_deserialization")]
    fn deserialize(bytes: &[u8]) -> Result<Self, tls_codec::Error> {
        Self::tls_deserialize_exact(bytes)
    }
}
```
"##)] # [cfg (feature = "conditional_deserialization")] # [proc_macro_attribute] pub fn conditionally_deserializable (_input : TokenStream , annotated_item : TokenStream ,) -> TokenStream { let annotated_item = parse_macro_input ! (annotated_item as ItemStruct) ; impl_conditionally_deserializable (annotated_item) . into () }
};
}
