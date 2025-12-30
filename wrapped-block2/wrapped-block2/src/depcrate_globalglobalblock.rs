// Generated macro for GlobalBlock (struct)
macro_rules! Depcrate_globalGlobalBlock {
() => {
// Module: crate::global
// Provides: {"GlobalBlock"}
// Dependencies: {}
# [doc = " A global Objective-C block that does not capture an environment."] # [doc = ""] # [doc = " This can be used as an optimization of [`RcBlock`] if your closure doesn't"] # [doc = " capture any variables."] # [doc = ""] # [doc = " This is a smart pointer that [`Deref`]s to [`Block`]."] # [doc = ""] # [doc = " It can created and stored in static memory using the [`global_block!`]"] # [doc = " macro."] # [doc = ""] # [doc = " [`RcBlock`]: crate::RcBlock"] # [doc = " [`global_block!`]: crate::global_block"] # [repr (C)] pub struct GlobalBlock < F : ? Sized > { header : BlockHeader , f : PhantomData < F > , }
};
}
