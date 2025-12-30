// Generated macro for Map (struct)
macro_rules! Depcrate_mapMap {
() => {
// Module: crate::map
// Provides: {"Map"}
// Dependencies: {}
# [doc = " An immutable map constructed at compile time."] # [doc = ""] # [doc = " ## Note"] # [doc = ""] # [doc = " The fields of this struct are public so that they may be initialized by the"] # [doc = " `phf_map!` macro and code generation. They are subject to change at any"] # [doc = " time and should never be accessed directly."] pub struct Map < K : 'static , V : 'static > { # [doc (hidden)] pub key : HashKey , # [doc (hidden)] pub disps : & 'static [(u32 , u32)] , # [doc (hidden)] pub entries : & 'static [(K , V)] , }
};
}
