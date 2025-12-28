macro_rules! deps {
    () => {
        CheckCase!();
        Error!();
    };
}

macro_rules! faster_hex_serde_option_macros {
    () => {
        deps!();
        # [doc = " Generate module with serde option methods"] macro_rules ! faster_hex_serde_option_macros { ($ mod_name : ident , $ with_pfx : expr , $ check_case : expr) => { # [doc = " Serialize and deserialize with or without 0x-prefix,"] # [doc = " and lowercase or uppercase or ignorecase for Option<Vec<u8>>"] pub mod $ mod_name { use crate :: decode :: CheckCase ; use crate :: serde :: internal ; use core :: iter :: FromIterator ; # [doc = " Serializes `Option<data>` as hex string or null"] pub fn serialize < S , T > (data : & Option < T >, serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , T : AsRef < [u8] >, { internal :: serialize_option (data , serializer , $ with_pfx , $ check_case) } # [doc = " Deserializes a hex string or null into `Option<Vec<u8>>`."] pub fn deserialize <'de , D , T > (deserializer : D) -> Result < Option < T >, D :: Error > where D : serde :: Deserializer <'de >, T : FromIterator < u8 >, { internal :: deserialize_option (deserializer , $ with_pfx , $ check_case) } } } ; }
    };
}

faster_hex_serde_option_macros!()