// Generated macro for AtomicDeserializer (struct)
macro_rules! Depcrate_de_simple_typeAtomicDeserializer {
() => {
// Module: crate::de::simple_type
// Provides: {"AtomicDeserializer"}
// Dependencies: {}
# [doc = " A deserializer that handles ordinary [simple type definition][item] with"] # [doc = " `{variety} = atomic`, or an ordinary [simple type] definition with"] # [doc = " `{variety} = union` whose basic members are all atomic."] # [doc = ""] # [doc = " This deserializer can deserialize only primitive types:"] # [doc = " - numbers"] # [doc = " - booleans"] # [doc = " - strings"] # [doc = " - units"] # [doc = " - options"] # [doc = " - unit variants of enums"] # [doc = ""] # [doc = " Identifiers represented as strings and deserialized accordingly."] # [doc = ""] # [doc = " Deserialization of all other types will provide a string and in most cases"] # [doc = " the deserialization will fail because visitor does not expect that."] # [doc = ""] # [doc = " The `Owned` variant of the content acts as a storage for data, allocated by"] # [doc = " an external deserializer that pass it via [`ListIter`]."] # [doc = ""] # [doc = " [item]: https://www.w3.org/TR/xmlschema11-1/#std-item_type_definition"] # [doc = " [simple type]: https://www.w3.org/TR/xmlschema11-1/#Simple_Type_Definition"] struct AtomicDeserializer < 'de , 'a > { # [doc = " Content of the attribute value, text content or CDATA content"] content : CowRef < 'de , 'a , str > , # [doc = " If `true`, `content` in an escaped form and should be unescaped before use"] escaped : bool , }
};
}
