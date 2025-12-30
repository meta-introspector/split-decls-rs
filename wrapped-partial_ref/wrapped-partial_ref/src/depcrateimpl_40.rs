// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
# [doc = " A nested part is itself a part."] impl < Outer , OuterFieldType , Inner > Part for Nested < Outer , Inner > where Outer : Part < PartType = Field < OuterFieldType > > , Inner : Part , OuterFieldType : ? Sized , OuterFieldType : HasPart < Inner > , OuterFieldType : PartialRefTarget < RawTarget = OuterFieldType > , { type PartType = Inner :: PartType ; }
};
}
