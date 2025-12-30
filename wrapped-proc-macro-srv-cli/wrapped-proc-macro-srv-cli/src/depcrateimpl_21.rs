// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl ValueEnum for ProtocolFormat { fn value_variants < 'a > () -> & 'a [Self] { & [ProtocolFormat :: Json] } fn to_possible_value (& self) -> Option < clap :: builder :: PossibleValue > { match self { ProtocolFormat :: Json => Some (clap :: builder :: PossibleValue :: new ("json")) , # [cfg (feature = "postcard")] ProtocolFormat :: Postcard => Some (clap :: builder :: PossibleValue :: new ("postcard")) , } } fn from_str (input : & str , _ignore_case : bool) -> Result < Self , String > { match input { "json" => Ok (ProtocolFormat :: Json) , # [cfg (feature = "postcard")] "postcard" => Ok (ProtocolFormat :: Postcard) , _ => Err (format ! ("unknown protocol format: {input}")) , } } }
};
}
