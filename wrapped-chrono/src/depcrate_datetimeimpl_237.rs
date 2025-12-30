// Generated macro for impl_237 (impl)
macro_rules! Depcrate_datetimeimpl_237 {
() => {
// Module: crate::datetime
// Provides: {"impl_237"}
// Dependencies: {}
# [cfg (feature = "rkyv-validation")] impl < Tz : TimeZone > fmt :: Debug for ArchivedDateTime < Tz > where Tz : Archive , < Tz as Archive > :: Archived : fmt :: Debug , < < Tz as TimeZone > :: Offset as Archive > :: Archived : fmt :: Debug , < Tz as TimeZone > :: Offset : fmt :: Debug + Archive , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("ArchivedDateTime") . field ("datetime" , & self . datetime) . field ("offset" , & self . offset) . finish () } }
};
}
