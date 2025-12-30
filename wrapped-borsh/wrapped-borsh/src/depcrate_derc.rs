// Generated macro for rc (module)
macro_rules! Depcrate_derc {
() => {
// Module: crate::de
// Provides: {"rc"}
// Dependencies: {}
# [doc = " Module is available if borsh is built with `features = [\"rc\"]`."] # [cfg (feature = "rc")] pub mod rc { # ! [doc = ""] # ! [doc = " Module defines [BorshDeserialize] implementation for"] # ! [doc = " [alloc::rc::Rc](std::rc::Rc) and [alloc::sync::Arc](std::sync::Arc)."] use crate :: __private :: maybestd :: { boxed :: Box , rc :: Rc , sync :: Arc } ; use crate :: io :: { Read , Result } ; use crate :: BorshDeserialize ; # [doc = " This impl requires the [`\"rc\"`] Cargo feature of borsh."] # [doc = ""] # [doc = " Deserializing a data structure containing `Rc` will not attempt to"] # [doc = " deduplicate `Rc` references to the same data. Every deserialized `Rc`"] # [doc = " will end up with a strong count of 1."] impl < T : ? Sized > BorshDeserialize for Rc < T > where Box < T > : BorshDeserialize , { fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { Ok (Box :: < T > :: deserialize_reader (reader) ? . into ()) } } # [doc = " This impl requires the [`\"rc\"`] Cargo feature of borsh."] # [doc = ""] # [doc = " Deserializing a data structure containing `Arc` will not attempt to"] # [doc = " deduplicate `Arc` references to the same data. Every deserialized `Arc`"] # [doc = " will end up with a strong count of 1."] impl < T : ? Sized > BorshDeserialize for Arc < T > where Box < T > : BorshDeserialize , { fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { Ok (Box :: < T > :: deserialize_reader (reader) ? . into ()) } } }
};
}
