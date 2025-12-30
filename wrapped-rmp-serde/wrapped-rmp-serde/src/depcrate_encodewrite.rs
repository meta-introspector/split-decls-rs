// Generated macro for write (function)
macro_rules! Depcrate_encodewrite {
() => {
// Module: crate::encode
// Provides: {"write"}
// Dependencies: {}
# [doc = " Serialize the given data structure as MessagePack into the I/O stream."] # [doc = " This function uses compact representation - structures as arrays"] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to fail."] # [inline] pub fn write < W , T > (wr : & mut W , val : & T) -> Result < () , Error > where W : Write + ? Sized , T : Serialize + ? Sized , { val . serialize (& mut Serializer :: new (wr)) }
};
}
