// Generated macro for impl_62 (impl)
macro_rules! Depcrate_errorimpl_62 {
() => {
// Module: crate::error
// Provides: {"impl_62"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { use Error :: * ; write ! (f , "{}" , match self { WontImplement => "This is a feature that PostCard will never implement" , NotYetImplemented => { "This is a feature that Postcard intends to support, but does not yet" } SerializeBufferFull => "The serialize buffer is full" , SerializeSeqLengthUnknown => "The length of a sequence must be known" , DeserializeUnexpectedEnd => "Hit the end of buffer, expected more data" , DeserializeBadVarint => { "Found a varint that didn't terminate. Is the usize too big for this platform?" } DeserializeBadBool => "Found a bool that wasn't 0 or 1" , DeserializeBadChar => "Found an invalid unicode char" , DeserializeBadUtf8 => "Tried to parse invalid utf-8" , DeserializeBadOption => "Found an Option discriminant that wasn't 0 or 1" , DeserializeBadEnum => "Found an enum discriminant that was > u32::max_value()" , DeserializeBadEncoding => "The original data was not well encoded" , DeserializeBadCrc => "Bad CRC while deserializing" , SerdeSerCustom => "Serde Serialization Error" , SerdeDeCustom => "Serde Deserialization Error" , CollectStrError => "Error while processing `collect_str` during serialization" , }) } }
};
}
