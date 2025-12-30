// Generated macro for expect_associated_value (function)
macro_rules! Depcrate_persist_dirty_cleanexpect_associated_value {
() => {
// Module: crate::persist::dirty_clean
// Provides: {"expect_associated_value"}
// Dependencies: {}
fn expect_associated_value (tcx : TyCtxt < '_ > , item : & MetaItemInner) -> Symbol { if let Some (value) = item . value_str () { value } else if let Some (ident) = item . ident () { tcx . dcx () . emit_fatal (errors :: AssociatedValueExpectedFor { span : item . span () , ident }) ; } else { tcx . dcx () . emit_fatal (errors :: AssociatedValueExpected { span : item . span () }) ; } }
};
}
