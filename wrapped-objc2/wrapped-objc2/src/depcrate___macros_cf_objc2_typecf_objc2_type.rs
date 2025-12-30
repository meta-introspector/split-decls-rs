// Generated macro for cf_objc2_type (macro)
macro_rules! Depcrate___macros_cf_objc2_typecf_objc2_type {
() => {
// Module: crate::__macros::cf_objc2_type
// Provides: {"cf_objc2_type"}
// Dependencies: {}
# [doc = " Helper macro for implementing [`objc2`][crate] traits for"] # [doc = " CoreFoundation-like types."] # [doc (hidden)] # [macro_export] macro_rules ! cf_objc2_type { (unsafe impl $ (<$ ($ generic : ident : ?$ sized : ident) ,* $ (,) ?>) ? RefEncode <$ encoding_name : literal > for $ ty : ty { }) => { unsafe impl $ (<$ ($ generic : ?$ sized) ,*>) ? $ crate :: encode :: RefEncode for $ ty { const ENCODING_REF : $ crate :: encode :: Encoding = $ crate :: encode :: Encoding :: Pointer (&$ crate :: encode :: Encoding :: Struct ($ encoding_name , & []) ,) ; } unsafe impl $ (<$ ($ generic : ?$ sized) ,*>) ? $ crate :: Message for $ ty { } impl $ (<$ ($ generic : ?$ sized) ,*>) ? $ crate :: __macros :: AsRef <$ crate :: runtime :: AnyObject > for $ ty { # [inline] fn as_ref (& self) -> &$ crate :: runtime :: AnyObject { unsafe { $ crate :: __macros :: transmute (self) } } } impl $ (<$ ($ generic : ?$ sized) ,*>) ? $ crate :: __macros :: Borrow <$ crate :: runtime :: AnyObject > for $ ty { # [inline] fn borrow (& self) -> &$ crate :: runtime :: AnyObject { < Self as $ crate :: __macros :: AsRef <$ crate :: runtime :: AnyObject >>:: as_ref (self) } } } ; }
};
}
