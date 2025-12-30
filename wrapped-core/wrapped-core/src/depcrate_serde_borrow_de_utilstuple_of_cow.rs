// Generated macro for tuple_of_cow (function)
macro_rules! Depcrate_serde_borrow_de_utilstuple_of_cow {
() => {
// Module: crate::serde_borrow_de_utils
// Provides: {"tuple_of_cow"}
// Dependencies: {}
pub fn tuple_of_cow < 'de , D > (deserializer : D) -> Result < (Cow < 'de , str > , Cow < 'de , str >) , D :: Error > where D : Deserializer < 'de > , (CowWrap < 'de > , CowWrap < 'de >) : Deserialize < 'de > , { < (CowWrap < 'de > , CowWrap < 'de >) > :: deserialize (deserializer) . map (| x | (x . 0 . 0 , x . 1 . 0)) }
};
}
