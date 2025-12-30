// Generated macro for add_padding (function)
macro_rules! Depcrate_xzadd_padding {
() => {
// Module: crate::xz
// Provides: {"add_padding"}
// Dependencies: {}
# [cfg (feature = "encoder")] fn add_padding < W : Write + ? Sized > (writer : & mut W , padding_needed : usize) -> crate :: Result < () > { match padding_needed { 1 => writer . write_all (& [0]) , 2 => writer . write_all (& [0 , 0]) , 3 => writer . write_all (& [0 , 0 , 0]) , _ => Ok (()) , } }
};
}
