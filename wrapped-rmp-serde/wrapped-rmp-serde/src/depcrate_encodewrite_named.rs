// Generated macro for write_named (function)
macro_rules! Depcrate_encodewrite_named {
() => {
// Module: crate::encode
// Provides: {"write_named"}
// Dependencies: {}
# [doc = " Serialize the given data structure as MessagePack into the I/O stream."] # [doc = " This function serializes structures as maps"] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to fail."] pub fn write_named < W , T > (wr : & mut W , val : & T) -> Result < () , Error > where W : Write + ? Sized , T : Serialize + ? Sized , { let mut se = Serializer :: new (wr) ; se . config = RuntimeConfig :: new (StructMapConfig :: new (se . config)) ; val . serialize (& mut se) }
};
}
