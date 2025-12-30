// Generated macro for Binding (trait)
macro_rules! Depcrate_utilBinding {
() => {
// Module: crate::util
// Provides: {"Binding"}
// Dependencies: {}
# [doc = " Provides access to the raw libgit2 pointer to be able to interact with libgit2-sys."] # [doc = ""] # [doc = " If you are going to depend on this trait on your code, do consider contributing to the git2"] # [doc = " project to add the missing capabilities to git2."] pub trait Binding : Sized { # [doc = " The raw type that allows you to interact with libgit2-sys."] type Raw ; # [doc = " Build a git2 struct from its [Binding::Raw] value."] unsafe fn from_raw (raw : Self :: Raw) -> Self ; # [doc = " Access the [Binding::Raw] value for a struct."] # [doc = ""] # [doc = " The returned value is only safe to use while its associated git2 struct is in scope."] # [doc = " Once the associated git2 struct is destroyed, the raw value can point to an invalid memory address."] fn raw (& self) -> Self :: Raw ; # [doc = " A null-handling version of [Binding::from_raw]."] # [doc = ""] # [doc = " If the input parameter is null, then the funtion returns None. Otherwise, it"] # [doc = " calls [Binding::from_raw]."] unsafe fn from_raw_opt < T > (raw : T) -> Option < Self > where T : Copy + IsNull , Self : Binding < Raw = T > , { if raw . is_ptr_null () { None } else { Some (Binding :: from_raw (raw)) } } }
};
}
