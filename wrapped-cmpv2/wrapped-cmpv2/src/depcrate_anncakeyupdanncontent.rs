// Generated macro for CaKeyUpdAnnContent (struct)
macro_rules! Depcrate_annCaKeyUpdAnnContent {
() => {
// Module: crate::ann
// Provides: {"CaKeyUpdAnnContent"}
// Dependencies: {}
# [doc = " The `CAKeyUpdAnnContent` announcement is defined in [RFC 4210 Section 5.3.13]."] # [doc = ""] # [doc = " ```text"] # [doc = "  CAKeyUpdAnnContent ::= SEQUENCE {"] # [doc = "      oldWithNew   CMPCertificate, -- old pub signed with new priv"] # [doc = "      newWithOld   CMPCertificate, -- new pub signed with old priv"] # [doc = "      newWithNew   CMPCertificate  -- new pub signed with new priv"] # [doc = "  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.13]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.13"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CaKeyUpdAnnContent { pub old_with_new : Box < CmpCertificate > , pub new_with_old : Box < CmpCertificate > , pub new_with_new : Box < CmpCertificate > , }
};
}
