macro_rules! deps {
    () => {
        Error!();
        CheckCase!();
    };
}

macro_rules! faster_hex_serde_macros {
    () => {
        deps!();
        # [doc = " Generate module with serde methods"] macro_rules ! faster_hex_serde_macros { ($ mod_name : ident , $ with_pfx : expr , $ check_case : expr) => { # [doc = " Serialize and deserialize with or without 0x-prefix,"] # [doc = " and lowercase or uppercase or ignorecase"] pub mod $ mod_name { use crate :: decode :: CheckCase ; use crate :: serde :: internal ; use core :: iter :: FromIterator ; # [doc = " Serializes `data` as hex string"] pub fn serialize < S , T > (data : T , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , T : AsRef < [u8] >, { internal :: serialize (data , serializer , $ with_pfx , $ check_case) } # [doc = " Deserializes a hex string into raw bytes."] pub fn deserialize <'de , D , T > (deserializer : D) -> Result < T , D :: Error > where D : serde :: Deserializer <'de >, T : FromIterator < u8 >, { internal :: deserialize (deserializer , $ with_pfx , $ check_case) } } } ; }
    };
}

faster_hex_serde_macros!();