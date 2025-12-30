// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
# [doc = " *(internal)* A part contains a nested part if it contains the outer part of the nested part."] impl < NestedPart , Outer , Inner , OuterFieldType , Index > ContainsNestedPart < Nested < Outer , Inner > , IndexNext < Index > > for NestedPart where NestedPart : Part , Inner : Part , NestedPart : ContainsNestedPart < Outer , Index > , Outer : Part < PartType = Field < OuterFieldType > > , OuterFieldType : ? Sized , OuterFieldType : HasPart < Inner > , OuterFieldType : PartialRefTarget < RawTarget = OuterFieldType > , { }
};
}
