// Generated macro for to_writer (function)
macro_rules! Depcrate_serto_writer {
() => {
// Module: crate::ser
// Provides: {"to_writer"}
// Dependencies: {}
# [doc = " Serializes `value` into `writer`."] # [doc = ""] # [doc = " This function does not generate any newlines or nice formatting;"] # [doc = " if you want that, you can use [`to_writer_pretty`] instead."] pub fn to_writer < W , T > (writer : W , value : & T) -> Result < () > where W : fmt :: Write , T : ? Sized + Serialize , { Options :: default () . to_writer (writer , value) }
};
}
