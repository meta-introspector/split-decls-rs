// Generated macro for EncodingType (trait)
macro_rules! Depcrate_coreEncodingType {
() => {
// Module: crate::core
// Provides: {"EncodingType"}
// Dependencies: {}
# [doc = " A trait for defining various supported encodings"] # [doc = " and implementing functionality that is encoding"] # [doc = " sensitive / specific."] pub trait EncodingType : private :: Sealed { type CodeUnit : PartialEq + core :: fmt :: Debug + Clone ; # [doc = " Get a slice from the underlying source using for start..end"] fn slice (source : & [Self :: CodeUnit] , start : usize , end : usize) -> Option < & [Self :: CodeUnit] > ; # [doc = " Retrieve the provided code unit index and returns the value as an ASCII byte"] # [doc = " or None if the value is not ASCII representable."] fn get_ascii (source : & [Self :: CodeUnit] , index : usize) -> ParserResult < Option < u8 > > ; # [doc = " Checks for the known calendar annotation key `u-ca`."] fn check_calendar_key (key : & [Self :: CodeUnit]) -> bool ; }
};
}
