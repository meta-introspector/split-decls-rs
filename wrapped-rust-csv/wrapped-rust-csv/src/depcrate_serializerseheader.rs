// Generated macro for SeHeader (struct)
macro_rules! Depcrate_serializerSeHeader {
() => {
// Module: crate::serializer
// Provides: {"SeHeader"}
// Dependencies: {}
struct SeHeader < 'w , W : 'w + io :: Write > { wtr : & 'w mut Writer < W > , state : HeaderState , }
};
}
