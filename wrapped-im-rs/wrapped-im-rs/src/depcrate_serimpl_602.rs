// Generated macro for impl_602 (impl)
macro_rules! Depcrate_serimpl_602 {
() => {
// Module: crate::ser
// Provides: {"impl_602"}
// Dependencies: {}
impl < 'de , S , K , V > Visitor < 'de > for MapVisitor < 'de , S , K , V > where S : From < Vec < (K , V) > > , K : Deserialize < 'de > , V : Deserialize < 'de > , { type Value = S ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a sequence") } fn visit_map < Access > (self , mut access : Access) -> Result < Self :: Value , Access :: Error > where Access : MapAccess < 'de > , { let mut v : Vec < (K , V) > = match access . size_hint () { None => Vec :: new () , Some (l) => Vec :: with_capacity (l) , } ; while let Some (i) = access . next_entry () ? { v . push (i) } Ok (From :: from (v)) } }
};
}
