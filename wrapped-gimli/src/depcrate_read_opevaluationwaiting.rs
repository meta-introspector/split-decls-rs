// Generated macro for EvaluationWaiting (enum)
macro_rules! Depcrate_read_opEvaluationWaiting {
() => {
// Module: crate::read::op
// Provides: {"EvaluationWaiting"}
// Dependencies: {}
# [derive (Debug)] enum EvaluationWaiting < R : Reader > { Memory , Register { offset : i64 } , FrameBase { offset : i64 } , Tls , Cfa , AtLocation , EntryValue , ParameterRef , RelocatedAddress , IndexedAddress , TypedLiteral { value : R } , Convert , Reinterpret , }
};
}
