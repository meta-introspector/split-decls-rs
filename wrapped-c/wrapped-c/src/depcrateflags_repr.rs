// Generated macro for flags_repr (function)
macro_rules! Depcrateflags_repr {
() => {
// Module: crate
// Provides: {"flags_repr"}
// Dependencies: {}
pub fn flags_repr (f : & Flags) -> Int { match f . repr () { FlagsRepr :: U8 => Int :: U8 , FlagsRepr :: U16 => Int :: U16 , FlagsRepr :: U32 (1) => Int :: U32 , FlagsRepr :: U32 (2) => Int :: U64 , repr => panic ! ("unimplemented flags {:?}" , repr) , } }
};
}
