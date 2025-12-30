// Generated macro for impl_180 (impl)
macro_rules! Depcrate___macros_define_class_ivarsimpl_180 {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"impl_180"}
// Dependencies: {}
impl < T : DefinedClass > DefinedIvarsHelper for T { # [doc = " Only add ivar if we need the runtime to allocate memory for it."] # [doc = ""] # [doc = " We can avoid doing so if the type is a zero-sized type (ZST), and the"] # [doc = " required alignment is less than the alignment of a pointer (objects"] # [doc = " are guaranteed to have at least that alignment themselves)."] const HAS_IVARS : bool = { mem :: size_of :: < T :: Ivars > () > 0 || mem :: align_of :: < T :: Ivars > () > mem :: align_of :: < * mut AnyObject > () } ; # [doc = " Only add drop flag if the type or the ivars need it."] # [doc = ""] # [doc = " `needs_drop::<T>` can reliably detect a direct implementation of"] # [doc = " `Drop`, since the type only includes `ManuallyDrop` or `PhantomData`"] # [doc = " fields."] const HAS_DROP_FLAG : bool = mem :: needs_drop :: < T > () || mem :: needs_drop :: < T :: Ivars > () ; }
};
}
