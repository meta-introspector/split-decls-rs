// Generated macro for should_panic (function)
macro_rules! Depcrate_testshould_panic {
() => {
// Module: crate::test
// Provides: {"should_panic"}
// Dependencies: {}
fn should_panic (cx : & ExtCtxt < '_ > , i : & ast :: Item) -> ShouldPanic { if let Some (Attribute :: Parsed (AttributeKind :: ShouldPanic { reason , .. })) = AttributeParser :: parse_limited (cx . sess , & i . attrs , sym :: should_panic , i . span , i . node_id () , None ,) { ShouldPanic :: Yes (reason) } else { ShouldPanic :: No } }
};
}
