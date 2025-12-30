// Generated macro for impl_233 (impl)
macro_rules! Depcrate_errorimpl_233 {
() => {
// Module: crate::error
// Provides: {"impl_233"}
// Dependencies: {}
impl fmt :: Debug for NSError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut debug = f . debug_struct ("NSError") ; debug . field ("code" , & self . code ()) ; # [cfg (feature = "NSString")] debug . field ("localizedDescription" , & self . localizedDescription ()) ; # [cfg (feature = "NSString")] debug . field ("domain" , & self . domain ()) ; # [cfg (all (feature = "NSDictionary" , feature = "NSString"))] debug . field ("userInfo" , & self . userInfo ()) ; debug . finish_non_exhaustive () } }
};
}
