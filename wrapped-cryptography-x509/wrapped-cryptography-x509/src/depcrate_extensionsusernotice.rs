// Generated macro for UserNotice (struct)
macro_rules! Depcrate_extensionsUserNotice {
() => {
// Module: crate::extensions
// Provides: {"UserNotice"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct UserNotice < 'a , Op : Asn1Operation > { pub notice_ref : Option < NoticeReference < 'a , Op > > , pub explicit_text : Option < DisplayText < 'a > > , }
};
}
