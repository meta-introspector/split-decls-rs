// Generated macro for impl_207 (impl)
macro_rules! Depcrate_dictionaryimpl_207 {
() => {
// Module: crate::dictionary
// Provides: {"impl_207"}
// Dependencies: {}
# [doc = " NSDictionaryCreation with known `NSString` key (because they load PLists)."] # [cfg (feature = "NSString")] impl < ObjectType : Message > NSDictionary < crate :: NSString , ObjectType > { objc2 :: extern_methods ! (# [cfg (all (feature = "NSError" , feature = "NSURL"))] # [unsafe (method (initWithContentsOfURL : error : _))] # [unsafe (method_family = init)] pub unsafe fn initWithContentsOfURL_error (this : objc2 :: rc :: Allocated < Self >, url : & crate :: NSURL ,) -> Result < Retained < Self >, Retained < crate :: NSError >>; # [cfg (all (feature = "NSError" , feature = "NSURL"))] # [unsafe (method (dictionaryWithContentsOfURL : error : _))] # [unsafe (method_family = none)] pub unsafe fn dictionaryWithContentsOfURL_error (url : & crate :: NSURL ,) -> Result < Retained < Self >, Retained < crate :: NSError >>;) ; }
};
}
