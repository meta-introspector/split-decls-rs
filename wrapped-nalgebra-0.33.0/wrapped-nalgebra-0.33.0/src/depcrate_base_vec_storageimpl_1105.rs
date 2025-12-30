// Generated macro for impl_1105 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1105 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1105"}
// Dependencies: {}
# [cfg (feature = "serde-serialize")] impl < 'a , T , R : Dim , C : Dim > Deserialize < 'a > for VecStorage < T , R , C > where T : Deserialize < 'a > , R : Deserialize < 'a > , C : Deserialize < 'a > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'a > , { let (data , nrows , ncols) : (Vec < T > , R , C) = Deserialize :: deserialize (deserializer) ? ; if nrows . value () * ncols . value () != data . len () { return Err (Des :: Error :: custom (format ! ("Expected {} components, found {}" , nrows . value () * ncols . value () , data . len ()))) ; } Ok (Self { data , nrows , ncols }) } }
};
}
