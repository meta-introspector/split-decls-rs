// Generated macro for lookup (function)
macro_rules! Depcratelookup {
() => {
// Module: crate
// Provides: {"lookup"}
// Dependencies: {}
# [doc = " Looks up an `IOSurface` by its global ID."] # [doc = ""] # [doc = " FIXME(pcwalton): This should return an `Option`."] pub fn lookup (csid : IOSurfaceID) -> IOSurface { unsafe { TCFType :: wrap_under_create_rule (IOSurfaceLookup (csid)) } }
};
}
