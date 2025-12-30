// Generated macro for from_trait (function)
macro_rules! Depcrate_defrom_trait {
() => {
// Module: crate::de
// Provides: {"from_trait"}
// Dependencies: {}
fn from_trait < 'de , R , T > (read : R) -> Result < T > where R : Read < 'de > , T : de :: Deserialize < 'de > , { let mut de = Deserializer :: new (read) ; let value = tri ! (de :: Deserialize :: deserialize (& mut de)) ; tri ! (de . end ()) ; Ok (value) }
};
}
