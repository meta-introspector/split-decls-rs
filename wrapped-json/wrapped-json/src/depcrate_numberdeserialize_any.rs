// Generated macro for deserialize_any (macro)
macro_rules! Depcrate_numberdeserialize_any {
() => {
// Module: crate::number
// Provides: {"deserialize_any"}
// Dependencies: {}
macro_rules ! deserialize_any { (@ expand [$ ($ num_string : tt) *]) => { # [cfg (not (feature = "arbitrary_precision"))] fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor <'de >, { match self . n { N :: PosInt (u) => visitor . visit_u64 (u) , N :: NegInt (i) => visitor . visit_i64 (i) , N :: Float (f) => visitor . visit_f64 (f) , } } # [cfg (feature = "arbitrary_precision")] fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor <'de > { if let Some (u) = self . as_u64 () { return visitor . visit_u64 (u) ; } else if let Some (i) = self . as_i64 () { return visitor . visit_i64 (i) ; } else if let Some (u) = self . as_u128 () { return visitor . visit_u128 (u) ; } else if let Some (i) = self . as_i128 () { return visitor . visit_i128 (i) ; } else if let Some (f) = self . as_f64 () { if ryu :: Buffer :: new () . format_finite (f) == self . n || f . to_string () == self . n { return visitor . visit_f64 (f) ; } } visitor . visit_map (NumberDeserializer { number : Some (self .$ ($ num_string) *) , }) } } ; (owned) => { deserialize_any ! (@ expand [n]) ; } ; (ref) => { deserialize_any ! (@ expand [n . clone ()]) ; } ; }
};
}
