// Generated macro for to_writer (function)
macro_rules! Depcrate_serto_writer {
() => {
// Module: crate::ser
// Provides: {"to_writer"}
// Dependencies: {}
# [doc = " Serializes a value to a writer."] # [cfg (feature = "std")] pub fn to_writer < W , T > (writer : W , value : & T) -> Result < () > where W : io :: Write , T : ser :: Serialize , { value . serialize (& mut Serializer :: new (& mut IoWrite :: new (writer))) }
};
}
