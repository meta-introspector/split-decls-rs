// Generated macro for to_writer_binary (function)
macro_rules! Depcrate_serto_writer_binary {
() => {
// Module: crate::ser
// Provides: {"to_writer_binary"}
// Dependencies: {}
# [doc = " Serializes the given data structure to a byte stream as a binary encoded plist."] pub fn to_writer_binary < W : Write , T : ser :: Serialize > (writer : W , value : & T) -> Result < () , Error > { let writer = stream :: BinaryWriter :: new (writer) ; let mut ser = Serializer :: new (writer) ; value . serialize (& mut ser) }
};
}
