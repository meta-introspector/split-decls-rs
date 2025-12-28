macro_rules! TreatNotYetDefinedOpaques {
    () => {
        # [doc = " Used by [FnCtxt::lookup_method_for_operator] with `-Znext-solver`."] # [doc = ""] # [doc = " With `AsRigid` we error on `impl Opaque: NotInItemBounds` while"] # [doc = " `AsInfer` just treats it as ambiguous and succeeds. This is necessary"] # [doc = " as we want [FnCtxt::check_expr_call] to treat not-yet-defined opaque"] # [doc = " types as rigid to support `impl Deref<Target = impl FnOnce()>` and"] # [doc = " `Box<impl FnOnce()>`."] # [doc = ""] # [doc = " We only want to treat opaque types as rigid if we need to eagerly choose"] # [doc = " between multiple candidates. We otherwise treat them as ordinary inference"] # [doc = " variable to avoid rejecting otherwise correct code."] # [derive (Debug)] # [expect (dead_code)] pub (super) enum TreatNotYetDefinedOpaques { AsInfer , AsRigid , }
    };
}

TreatNotYetDefinedOpaques!();