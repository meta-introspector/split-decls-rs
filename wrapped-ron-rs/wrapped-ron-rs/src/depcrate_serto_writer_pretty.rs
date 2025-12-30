// Generated macro for to_writer_pretty (function)
macro_rules! Depcrate_serto_writer_pretty {
() => {
// Module: crate::ser
// Provides: {"to_writer_pretty"}
// Dependencies: {}
# [doc = " Serializes `value` into `writer` in a pretty way."] pub fn to_writer_pretty < W , T > (writer : W , value : & T , config : PrettyConfig) -> Result < () > where W : fmt :: Write , T : ? Sized + Serialize , { Options :: default () . to_writer_pretty (writer , value , config) }
};
}
