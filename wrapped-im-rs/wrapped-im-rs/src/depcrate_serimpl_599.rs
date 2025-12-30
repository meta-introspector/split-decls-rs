// Generated macro for impl_599 (impl)
macro_rules! Depcrate_serimpl_599 {
() => {
// Module: crate::ser
// Provides: {"impl_599"}
// Dependencies: {}
impl < 'de , S , A > Visitor < 'de > for SeqVisitor < 'de , S , A > where S : From < Vec < A > > , A : Deserialize < 'de > , { type Value = S ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < Access > (self , mut access : Access) -> Result < Self :: Value , Access :: Error > where Access : SeqAccess < 'de > , { let mut v : Vec < A > = match access . size_hint () { None => Vec :: new () , Some (l) => Vec :: with_capacity (l) , } ; while let Some (i) = access . next_element () ? { v . push (i) } Ok (From :: from (v)) } }
};
}
