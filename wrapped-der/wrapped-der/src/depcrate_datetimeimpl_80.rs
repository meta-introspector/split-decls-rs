// Generated macro for impl_80 (impl)
macro_rules! Depcrate_datetimeimpl_80 {
() => {
// Module: crate::datetime
// Provides: {"impl_80"}
// Dependencies: {}
# [cfg (feature = "time")] impl TryFrom < PrimitiveDateTime > for DateTime { type Error = Error ; fn try_from (time : PrimitiveDateTime) -> Result < DateTime > { DateTime :: new (time . year () . try_into () . map_err (| _ | ErrorKind :: DateTime) ? , time . month () . into () , time . day () , time . hour () , time . minute () , time . second () ,) } }
};
}
