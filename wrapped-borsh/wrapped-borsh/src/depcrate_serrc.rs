// Generated macro for rc (module)
macro_rules! Depcrate_serrc {
() => {
// Module: crate::ser
// Provides: {"rc"}
// Dependencies: {}
# [doc = " Module is available if borsh is built with `features = [\"rc\"]`."] # [cfg (feature = "rc")] pub mod rc { # ! [doc = ""] # ! [doc = " Module defines [BorshSerialize] implementation for"] # ! [doc = " [alloc::rc::Rc](std::rc::Rc) and [alloc::sync::Arc](std::sync::Arc)."] use crate :: __private :: maybestd :: { rc :: Rc , sync :: Arc } ; use crate :: io :: { Result , Write } ; use crate :: BorshSerialize ; # [doc = " This impl requires the [`\"rc\"`] Cargo feature of borsh."] # [doc = ""] # [doc = " Serializing a data structure containing `Rc` will serialize a copy of"] # [doc = " the contents of the `Rc` each time the `Rc` is referenced within the"] # [doc = " data structure. Serialization will not attempt to deduplicate these"] # [doc = " repeated data."] impl < T : BorshSerialize + ? Sized > BorshSerialize for Rc < T > { fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { (* * self) . serialize (writer) } } # [doc = " This impl requires the [`\"rc\"`] Cargo feature of borsh."] # [doc = ""] # [doc = " Serializing a data structure containing `Arc` will serialize a copy of"] # [doc = " the contents of the `Arc` each time the `Arc` is referenced within the"] # [doc = " data structure. Serialization will not attempt to deduplicate these"] # [doc = " repeated data."] impl < T : BorshSerialize + ? Sized > BorshSerialize for Arc < T > { fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { (* * self) . serialize (writer) } } }
};
}
