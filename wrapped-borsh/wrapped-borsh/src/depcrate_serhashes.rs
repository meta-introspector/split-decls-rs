// Generated macro for hashes (module)
macro_rules! Depcrate_serhashes {
() => {
// Module: crate::ser
// Provides: {"hashes"}
// Dependencies: {}
# [doc = " Module is available if borsh is built with `features = [\"std\"]` or `features = [\"hashbrown\"]`."] # [doc = ""] # [doc = " Module defines [BorshSerialize] implementation for"] # [doc = " [HashMap](std::collections::HashMap)/[HashSet](std::collections::HashSet)."] # [cfg (hash_collections)] pub mod hashes { use crate :: __private :: maybestd :: vec :: Vec ; use crate :: error :: check_zst ; use crate :: { BorshSerialize , __private :: maybestd :: collections :: { HashMap , HashSet } , } ; use core :: convert :: TryFrom ; use core :: hash :: BuildHasher ; use crate :: io :: { ErrorKind , Result , Write } ; impl < K , V , H > BorshSerialize for HashMap < K , V , H > where K : BorshSerialize + Ord , V : BorshSerialize , H : BuildHasher , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < K > () ? ; let mut vec = self . iter () . collect :: < Vec < _ > > () ; vec . sort_by (| (a , _) , (b , _) | a . cmp (b)) ; u32 :: try_from (vec . len ()) . map_err (| _ | ErrorKind :: InvalidData) ? . serialize (writer) ? ; for kv in vec { kv . serialize (writer) ? ; } Ok (()) } } impl < T , H > BorshSerialize for HashSet < T , H > where T : BorshSerialize + Ord , H : BuildHasher , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < T > () ? ; let mut vec = self . iter () . collect :: < Vec < _ > > () ; vec . sort () ; u32 :: try_from (vec . len ()) . map_err (| _ | ErrorKind :: InvalidData) ? . serialize (writer) ? ; for item in vec { item . serialize (writer) ? ; } Ok (()) } } }
};
}
