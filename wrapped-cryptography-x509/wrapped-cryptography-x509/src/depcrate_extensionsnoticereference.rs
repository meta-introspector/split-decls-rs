// Generated macro for NoticeReference (struct)
macro_rules! Depcrate_extensionsNoticeReference {
() => {
// Module: crate::extensions
// Provides: {"NoticeReference"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct NoticeReference < 'a , Op : Asn1Operation > { pub organization : DisplayText < 'a > , pub notice_numbers : Op :: SequenceOfVec < 'a , asn1 :: BigUint < 'a > > , }
};
}
