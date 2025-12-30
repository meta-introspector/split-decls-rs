// Generated macro for get_bodies_with_borrowck_facts (function)
macro_rules! Depcrate_consumersget_bodies_with_borrowck_facts {
() => {
// Module: crate::consumers
// Provides: {"get_bodies_with_borrowck_facts"}
// Dependencies: {}
# [doc = " This function computes borrowck facts for the given def id and all its nested bodies."] # [doc = " It must be called with a typeck root which will then borrowck all nested bodies as well."] # [doc = " The [`ConsumerOptions`] determine which facts are returned. This function makes a copy"] # [doc = " of the bodies because it needs to regenerate the region identifiers. It should never be"] # [doc = " invoked during a typical compilation session due to the unnecessary overhead of"] # [doc = " returning [`BodyWithBorrowckFacts`]."] # [doc = ""] # [doc = " Note:"] # [doc = " *   This function will panic if the required bodies were already stolen. This"] # [doc = "     can, for example, happen when requesting a body of a `const` function"] # [doc = "     because they are evaluated during typechecking. The panic can be avoided"] # [doc = "     by overriding the `mir_borrowck` query. You can find a complete example"] # [doc = "     that shows how to do this at `tests/ui-fulldeps/obtain-borrowck.rs`."] # [doc = ""] # [doc = " *   Polonius is highly unstable, so expect regular changes in its signature or other details."] pub fn get_bodies_with_borrowck_facts (tcx : TyCtxt < '_ > , root_def_id : LocalDefId , options : ConsumerOptions ,) -> FxHashMap < LocalDefId , BodyWithBorrowckFacts < '_ > > { let mut root_cx = BorrowCheckRootCtxt :: new (tcx , root_def_id , Some (BorrowckConsumer :: new (options))) ; root_cx . do_mir_borrowck () ; root_cx . consumer . unwrap () . bodies }
};
}
