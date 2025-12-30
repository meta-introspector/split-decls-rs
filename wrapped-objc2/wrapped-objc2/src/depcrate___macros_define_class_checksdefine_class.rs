// Generated macro for define_class (function)
macro_rules! Depcrate___macros_define_class_checksdefine_class {
() => {
// Module: crate::__macros::define_class::checks
// Provides: {"define_class"}
// Dependencies: {}
# [inline] # [track_caller] # [allow (clippy :: new_without_default)] pub fn define_class < T : DefinedClass > (c_name : & CStr , name_is_auto_generated : bool , register_impls : impl FnOnce (& mut ClassBuilderHelper < T >) ,) -> (& 'static AnyClass , isize , isize) where T :: Super : ClassType , { let (ivar_name , drop_flag_name) = ivar_drop_flag_names :: < T > () ; let superclass = < T :: Super as ClassType > :: class () ; let cls = if let Some (builder) = ClassBuilder :: new (c_name , superclass) { let mut this = ClassBuilderHelper { builder , p : PhantomData , } ; setup_dealloc :: < T > (& mut this . builder) ; register_impls (& mut this) ; register_ivars :: < T > (& mut this . builder , & ivar_name) ; register_drop_flag :: < T > (& mut this . builder , & drop_flag_name) ; this . builder . register () } else { let overridden = option_env ! ("UNSAFE_OBJC2_ALLOW_CLASS_OVERRIDE") == Some ("1") ; if name_is_auto_generated || overridden { AnyClass :: get (c_name) . unwrap_or_else (| | class_not_present (c_name)) } else { class_not_unique (c_name) } } ; (cls , ivars_offset :: < T > (cls , & ivar_name) , drop_flag_offset :: < T > (cls , & drop_flag_name) ,) }
};
}
