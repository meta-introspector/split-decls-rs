// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
# [doc = " *(internal)* Pluck a constant nested subpart of the constant first part."] unsafe impl < 'a , ContainingPart , PluckedOuter , PluckedInner , Reference , NestedPartIndex , Index , OuterFieldType , ContainingFieldType , > PluckConst < 'a , Nested < PluckedOuter , PluckedInner > , IndexSplit < NestedPartIndex , Index > > for Const < ContainingPart , Reference > where PluckedOuter : Part < PartType = Field < OuterFieldType > > , PluckedInner : Part , ContainingPart : Part < PartType = Field < ContainingFieldType > > , ContainingFieldType : SplitIntoParts < 'a , ContainingPart , Reference > , ContainingFieldType : ? Sized , ContainingFieldType :: ResultMut : PluckConst < 'a , Nested < PluckedOuter , PluckedInner > , Index > , OuterFieldType : ? Sized , OuterFieldType : HasPart < PluckedInner > , OuterFieldType : PartialRefTarget < RawTarget = OuterFieldType > , Reference : PartialRef < 'a > , Reference :: Target : HasPart < ContainingPart > , ContainingPart : ContainsNestedPart < PluckedOuter , NestedPartIndex > , { type Remainder = Const < ContainingPart , Reference > ; }
};
}
