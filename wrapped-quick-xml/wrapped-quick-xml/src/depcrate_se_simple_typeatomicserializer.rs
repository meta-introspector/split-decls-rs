// Generated macro for AtomicSerializer (struct)
macro_rules! Depcrate_se_simple_typeAtomicSerializer {
() => {
// Module: crate::se::simple_type
// Provides: {"AtomicSerializer"}
// Dependencies: {}
# [doc = " A serializer that handles ordinary [simple type definition][item] with"] # [doc = " `{variety} = atomic`, or an ordinary [simple type] definition with"] # [doc = " `{variety} = union` whose basic members are all atomic."] # [doc = ""] # [doc = " This serializer can serialize only primitive types:"] # [doc = " - numbers"] # [doc = " - booleans"] # [doc = " - strings"] # [doc = " - units"] # [doc = " - options"] # [doc = " - unit variants of enums"] # [doc = ""] # [doc = " Identifiers represented as strings and serialized accordingly."] # [doc = ""] # [doc = " Serialization of all other types returns [`Unsupported`][SeError::Unsupported] error."] # [doc = ""] # [doc = " This serializer returns `true` if something was written and `false` otherwise."] # [doc = ""] # [doc = " [item]: https://www.w3.org/TR/xmlschema11-1/#std-item_type_definition"] # [doc = " [simple type]: https://www.w3.org/TR/xmlschema11-1/#Simple_Type_Definition"] pub struct AtomicSerializer < W : Write > { pub writer : W , pub target : QuoteTarget , # [doc = " Defines which XML characters need to be escaped"] pub level : QuoteLevel , # [doc = " When `true` an `xs:list` delimiter (a space) should be written"] pub (crate) write_delimiter : bool , }
};
}
