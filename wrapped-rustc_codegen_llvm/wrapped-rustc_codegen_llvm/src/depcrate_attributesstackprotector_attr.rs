// Generated macro for stackprotector_attr (function)
macro_rules! Depcrate_attributesstackprotector_attr {
() => {
// Module: crate::attributes
// Provides: {"stackprotector_attr"}
// Dependencies: {}
fn stackprotector_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { let sspattr = match cx . sess () . stack_protector () { StackProtector :: None => return None , StackProtector :: All => AttributeKind :: StackProtectReq , StackProtector :: Strong => AttributeKind :: StackProtectStrong , StackProtector :: Basic => AttributeKind :: StackProtect , } ; Some (sspattr . create_attr (cx . llcx)) }
};
}
