macro_rules! FeedConstTy {
    () => {
        # [doc = " In some cases, [`hir::ConstArg`]s that are being used in the type system"] # [doc = " through const generics need to have their type \"fed\" to them"] # [doc = " using the query system."] # [doc = ""] # [doc = " Use this enum with `<dyn HirTyLowerer>::lower_const_arg` to instruct it with the"] # [doc = " desired behavior."] # [derive (Debug , Clone , Copy)] pub enum FeedConstTy < 'a , 'tcx > { # [doc = " Feed the type."] # [doc = ""] # [doc = " The `DefId` belongs to the const param that we are supplying"] # [doc = " this (anon) const arg to."] # [doc = ""] # [doc = " The list of generic args is used to instantiate the parameters"] # [doc = " used by the type of the const param specified by `DefId`."] Param (DefId , & 'a [ty :: GenericArg < 'tcx >]) , # [doc = " Don't feed the type."] No , }
    };
}

FeedConstTy!()