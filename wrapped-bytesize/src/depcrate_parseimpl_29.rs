// Generated macro for impl_29 (impl)
macro_rules! Depcrate_parseimpl_29 {
() => {
// Module: crate::parse
// Provides: {"impl_29"}
// Dependencies: {}
impl str :: FromStr for Unit { type Err = String ; fn from_str (unit : & str) -> Result < Self , Self :: Err > { match unit . to_lowercase () . as_str () { "b" => Ok (Self :: Byte) , "k" | "kb" => Ok (Self :: KiloByte) , "m" | "mb" => Ok (Self :: MegaByte) , "g" | "gb" => Ok (Self :: GigaByte) , "t" | "tb" => Ok (Self :: TeraByte) , "p" | "pb" => Ok (Self :: PetaByte) , "e" | "eb" => Ok (Self :: ExaByte) , "ki" | "kib" => Ok (Self :: KibiByte) , "mi" | "mib" => Ok (Self :: MebiByte) , "gi" | "gib" => Ok (Self :: GibiByte) , "ti" | "tib" => Ok (Self :: TebiByte) , "pi" | "pib" => Ok (Self :: PebiByte) , "ei" | "eib" => Ok (Self :: ExbiByte) , _ => Err (format ! ("couldn't parse unit of {unit:?}")) , } } }
};
}
