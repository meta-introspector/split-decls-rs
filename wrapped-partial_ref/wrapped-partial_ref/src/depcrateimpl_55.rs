// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
# [doc = " *(internal)* Pluck a mutable nested subpart of the mutable first part."] # [doc = ""] # [doc = " This leaves all other subparts in the remaining reference."] unsafe impl < 'a , ContainingPart , PluckedOuter , PluckedInner , Reference , NestedPartIndex , Index , OuterFieldType , ContainingFieldType , > PluckMut < 'a , Nested < PluckedOuter , PluckedInner > , IndexSplit < NestedPartIndex , Index > > for Mut < ContainingPart , Reference > where PluckedOuter : Part < PartType = Field < OuterFieldType > > , PluckedInner : Part , ContainingPart : Part < PartType = Field < ContainingFieldType > > , ContainingFieldType : SplitIntoParts < 'a , ContainingPart , Reference > , ContainingFieldType : ? Sized , ContainingFieldType :: ResultMut : PluckMut < 'a , Nested < PluckedOuter , PluckedInner > , Index > , OuterFieldType : ? Sized , OuterFieldType : HasPart < PluckedInner > , OuterFieldType : PartialRefTarget < RawTarget = OuterFieldType > , Reference : PartialRef < 'a > , Reference :: Target : HasPart < ContainingPart > , ContainingPart : ContainsNestedPart < PluckedOuter , NestedPartIndex > , { type Remainder = < ContainingFieldType :: ResultMut as PluckMut < 'a , Nested < PluckedOuter , PluckedInner > , Index , > > :: Remainder ; }
};
}
