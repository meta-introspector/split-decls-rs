// Generated macro for array_of_cow (function)
macro_rules! Depcrate_serde_borrow_de_utilsarray_of_cow {
() => {
// Module: crate::serde_borrow_de_utils
// Provides: {"array_of_cow"}
// Dependencies: {}
pub fn array_of_cow < 'de , D , const N : usize > (deserializer : D) -> Result < [Cow < 'de , str > ; N] , D :: Error > where D : Deserializer < 'de > , [CowWrap < 'de > ; N] : Deserialize < 'de > , { < [CowWrap < 'de > ; N] > :: deserialize (deserializer) . map (| array | array . map (| wrap | wrap . 0)) }
};
}
