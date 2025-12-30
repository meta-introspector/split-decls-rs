// Generated macro for impl_584 (impl)
macro_rules! Depcrate_exceptionimpl_584 {
() => {
// Module: crate::exception
// Provides: {"impl_584"}
// Dependencies: {}
impl fmt :: Debug for Exception { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "exception ") ? ; if let Some (true) = self . is_nsexception () { autoreleasepool_leaking (| pool | { let (name , reason) = unsafe { (self . name () , self . reason ()) } ; let name = name . as_deref () . map (| name | unsafe { nsstring_to_str (name , pool) }) ; let reason = reason . as_deref () . map (| reason | unsafe { nsstring_to_str (reason , pool) }) ; let obj : & AnyObject = self . as_ref () ; write ! (f , "{obj:?} '{}'" , name . unwrap_or_default ()) ? ; if let Some (reason) = reason { write ! (f , " reason: {reason}") ? ; } else { write ! (f , " reason: (NULL)") ? ; } Ok (()) }) } else { write ! (f , "{:?}" , self . 0) } } }
};
}
