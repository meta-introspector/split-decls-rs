// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
# [doc = " A reference has a nested part if it has the outer part and the nested part is valid."] impl < Reference , Outer , OuterFieldType , Inner > HasPart < Nested < Outer , Inner > > for Reference where Reference : ? Sized , Reference : HasPart < Outer > , Outer : Part < PartType = Field < OuterFieldType > > , Inner : Part , OuterFieldType : ? Sized , OuterFieldType : HasPart < Inner > , OuterFieldType : PartialRefTarget < RawTarget = OuterFieldType > , { # [inline (always)] unsafe fn part_ptr (ptr : * const Self :: RawTarget) -> < Inner :: PartType as PartType > :: Ptr { < OuterFieldType as HasPart < Inner > > :: part_ptr (< Self as HasPart < Outer > > :: part_ptr (ptr)) } # [inline (always)] unsafe fn part_ptr_mut (ptr : * mut Self :: RawTarget) -> < Inner :: PartType as PartType > :: PtrMut { < OuterFieldType as HasPart < Inner > > :: part_ptr_mut (< Self as HasPart < Outer > > :: part_ptr_mut (ptr ,)) } }
};
}
