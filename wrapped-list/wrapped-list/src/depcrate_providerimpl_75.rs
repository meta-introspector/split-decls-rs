// Generated macro for impl_75 (impl)
macro_rules! Depcrate_providerimpl_75 {
() => {
// Module: crate::provider
// Provides: {"impl_75"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de : 'data , 'data > serde :: Deserialize < 'de > for ListJoinerPattern < 'data > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { # [derive (serde :: Deserialize)] struct Dummy < 'data > { # [cfg_attr (feature = "serde" , serde (borrow))] string : VarZeroCow < 'data , str > , index_1 : u8 , } let Dummy { string , index_1 } = Dummy :: deserialize (deserializer) ? ; if index_1 as usize > string . len () { use serde :: de :: Error ; Err (D :: Error :: custom ("invalid index_1")) } else { Ok (ListJoinerPattern { string , index_0 : 0 , index_1 , }) } } }
};
}
