// Generated macro for PackageHeader (struct)
macro_rules! Depcrate_hiiPackageHeader {
() => {
// Module: crate::hii
// Provides: {"PackageHeader"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct PackageHeader < const N : usize = 0 > { pub length : [u8 ; 3] , pub r#type : u8 , pub data : [u8 ; N] , }
};
}
