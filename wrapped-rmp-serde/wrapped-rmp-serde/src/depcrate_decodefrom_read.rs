// Generated macro for from_read (function)
macro_rules! Depcrate_decodefrom_read {
() => {
// Module: crate::decode
// Provides: {"from_read"}
// Dependencies: {}
# [doc = " Deserialize an instance of type `T` from an I/O stream of MessagePack."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This conversion can fail if the structure of the Value does not match the structure expected"] # [doc = " by `T`. It can also fail if the structure is correct but `T`'s implementation of `Deserialize`"] # [doc = " decides that something is wrong with the data, for example required struct fields are missing."] # [inline] pub fn from_read < R , T > (rd : R) -> Result < T , Error > where R : Read , T : DeserializeOwned { Deserialize :: deserialize (& mut Deserializer :: new (rd)) }
};
}
