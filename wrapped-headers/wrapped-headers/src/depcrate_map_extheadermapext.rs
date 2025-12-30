// Generated macro for HeaderMapExt (trait)
macro_rules! Depcrate_map_extHeaderMapExt {
() => {
// Module: crate::map_ext
// Provides: {"HeaderMapExt"}
// Dependencies: {}
# [doc = " An extension trait adding \"typed\" methods to `http::HeaderMap`."] pub trait HeaderMapExt : self :: sealed :: Sealed { # [doc = " Inserts the typed `Header` into this `HeaderMap`."] fn typed_insert < H > (& mut self , header : H) where H : Header ; # [doc = " Tries to find the header by name, and then decode it into `H`."] fn typed_get < H > (& self) -> Option < H > where H : Header ; # [doc = " Tries to find the header by name, and then decode it into `H`."] fn typed_try_get < H > (& self) -> Result < Option < H > , Error > where H : Header ; }
};
}
