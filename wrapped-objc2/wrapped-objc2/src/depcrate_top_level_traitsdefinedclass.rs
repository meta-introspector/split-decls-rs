// Generated macro for DefinedClass (trait)
macro_rules! Depcrate_top_level_traitsDefinedClass {
() => {
// Module: crate::top_level_traits
// Provides: {"DefinedClass"}
// Dependencies: {}
# [doc = " Marks class types whose implementation is defined in Rust."] # [doc = ""] # [doc = " This is used in [`define_class!`], and allows access to the instance"] # [doc = " variables that a given type declares, see that macro for details."] # [doc = ""] # [doc = " [`define_class!`]: crate::define_class"] pub trait DefinedClass : ClassType { # [doc = " A type representing the instance variables that this class carries."] type Ivars : Sized ; # [doc = " Get a reference to the instance variable data that this object"] # [doc = " carries."] # [inline] # [track_caller] # [doc (hidden)] fn __get_ivars (& self) -> & Self :: Ivars where Self : Sized , { let ptr : NonNull < Self > = NonNull :: from (self) ; let ivars = unsafe { get_initialized_ivar_ptr (ptr) } ; unsafe { ivars . as_ref () } } # [doc (hidden)] fn __ivars_offset () -> isize ; # [doc (hidden)] fn __drop_flag_offset () -> isize ; # [doc = " # Safety"] # [doc = ""] # [doc = " The ivar offset and drop flag offsets must be implemented correctly."] # [doc (hidden)] const __UNSAFE_OFFSETS_CORRECT : () ; }
};
}
