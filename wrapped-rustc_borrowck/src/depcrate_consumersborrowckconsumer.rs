// Generated macro for BorrowckConsumer (struct)
macro_rules! Depcrate_consumersBorrowckConsumer {
() => {
// Module: crate::consumers
// Provides: {"BorrowckConsumer"}
// Dependencies: {}
# [doc = " Struct used during mir borrowck to collect bodies with facts for a typeck root and all"] # [doc = " its nested bodies."] pub (crate) struct BorrowckConsumer < 'tcx > { options : ConsumerOptions , bodies : FxHashMap < LocalDefId , BodyWithBorrowckFacts < 'tcx > > , }
};
}
