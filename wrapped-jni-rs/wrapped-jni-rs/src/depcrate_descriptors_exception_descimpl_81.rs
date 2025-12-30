// Generated macro for impl_81 (impl)
macro_rules! Depcrate_descriptors_exception_descimpl_81 {
() => {
// Module: crate::descriptors::exception_desc
// Provides: {"impl_81"}
// Dependencies: {}
unsafe impl < 'local , 'other_local , C , M > Desc < 'local , JThrowable < 'local > > for (C , M) where C : Desc < 'local , JClass < 'other_local > > , M : AsRef < JNIStr > , { type Output = Auto < 'local , JThrowable < 'local > > ; fn lookup (self , env : & mut Env < 'local >) -> Result < Self :: Output > { let jmsg = env . new_string (self . 1 . as_ref ()) ? . auto () ; let obj : JObject = env . new_object (self . 0 , c"(Ljava/lang/String;)V" , & [JValue :: from (& jmsg)]) ? ; let throwable = env . cast_local :: < JThrowable > (obj) ? ; Ok (throwable . auto ()) } }
};
}
