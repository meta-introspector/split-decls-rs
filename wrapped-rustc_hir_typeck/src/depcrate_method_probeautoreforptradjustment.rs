// Generated macro for AutorefOrPtrAdjustment (enum)
macro_rules! Depcrate_method_probeAutorefOrPtrAdjustment {
() => {
// Module: crate::method::probe
// Provides: {"AutorefOrPtrAdjustment"}
// Dependencies: {}
# [doc = " When adjusting a receiver we often want to do one of"] # [doc = ""] # [doc = " - Add a `&` (or `&mut`), converting the receiver from `T` to `&T` (or `&mut T`)"] # [doc = " - If the receiver has type `*mut T`, convert it to `*const T`"] # [doc = ""] # [doc = " This type tells us which one to do."] # [doc = ""] # [doc = " Note that in principle we could do both at the same time. For example, when the receiver has"] # [doc = " type `T`, we could autoref it to `&T`, then convert to `*const T`. Or, when it has type `*mut"] # [doc = " T`, we could convert it to `*const T`, then autoref to `&*const T`. However, currently we do"] # [doc = " (at most) one of these. Either the receiver has type `T` and we convert it to `&T` (or with"] # [doc = " `mut`), or it has type `*mut T` and we convert it to `*const T`."] # [derive (Debug , PartialEq , Copy , Clone)] pub (crate) enum AutorefOrPtrAdjustment { # [doc = " Receiver has type `T`, add `&` or `&mut` (if `T` is `mut`), and maybe also \"unsize\" it."] # [doc = " Unsizing is used to convert a `[T; N]` to `[T]`, which only makes sense when autorefing."] Autoref { mutbl : hir :: Mutability , # [doc = " Indicates that the source expression should be \"unsized\" to a target type."] # [doc = " This is special-cased for just arrays unsizing to slices."] unsize : bool , } , # [doc = " Receiver has type `*mut T`, convert to `*const T`"] ToConstPtr , # [doc = " Reborrow a `Pin<&mut T>` or `Pin<&T>`."] ReborrowPin (hir :: Mutability) , }
};
}
