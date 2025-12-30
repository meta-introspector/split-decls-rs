// Generated macro for MemberHeader (enum)
macro_rules! Depcrate_read_archiveMemberHeader {
() => {
// Module: crate::read::archive
// Provides: {"MemberHeader"}
// Dependencies: {}
# [doc = " An archive member header."] # [derive (Debug , Clone , Copy)] enum MemberHeader < 'data > { # [doc = " Common header used by many formats."] Common (& 'data archive :: Header) , # [doc = " AIX big archive header"] AixBig (& 'data archive :: AixHeader) , }
};
}
