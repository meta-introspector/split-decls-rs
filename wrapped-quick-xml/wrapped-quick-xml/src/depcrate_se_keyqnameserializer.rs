// Generated macro for QNameSerializer (struct)
macro_rules! Depcrate_se_keyQNameSerializer {
() => {
// Module: crate::se::key
// Provides: {"QNameSerializer"}
// Dependencies: {}
# [doc = " A serializer, that ensures, that only plain types can be serialized,"] # [doc = " so result can be used as an XML tag or attribute name."] # [doc = ""] # [doc = " This serializer does not check that name does not contain characters that"] # [doc = " [not allowed] in XML names, because in some cases it should pass names"] # [doc = " that would be filtered on higher level."] # [doc = ""] # [doc = " [not allowed]: https://www.w3.org/TR/xml11/#sec-common-syn"] pub struct QNameSerializer < W : Write > { # [doc = " Writer to which this serializer writes content"] pub writer : W , }
};
}
