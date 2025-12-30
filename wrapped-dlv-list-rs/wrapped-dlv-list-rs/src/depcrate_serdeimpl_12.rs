// Generated macro for impl_12 (impl)
macro_rules! Depcrate_serdeimpl_12 {
() => {
// Module: crate::serde
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'de , T : Deserialize < 'de > > Visitor < 'de > for VecListVisitor < T > { type Value = VecList < T > ; fn expecting (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { write ! (formatter , "a sequence") } fn visit_seq < A > (self , mut access : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut list = VecList :: with_capacity (access . size_hint () . unwrap_or_default ()) ; while let Some (value) = access . next_element () ? { let _ = list . push_back (value) ; } Ok (list) } }
};
}
