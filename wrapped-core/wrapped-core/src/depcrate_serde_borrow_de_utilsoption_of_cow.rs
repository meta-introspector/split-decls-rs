// Generated macro for option_of_cow (function)
macro_rules! Depcrate_serde_borrow_de_utilsoption_of_cow {
() => {
// Module: crate::serde_borrow_de_utils
// Provides: {"option_of_cow"}
// Dependencies: {}
pub fn option_of_cow < 'de , D > (deserializer : D) -> Result < Option < Cow < 'de , str > > , D :: Error > where D : Deserializer < 'de > , { < Option < CowWrap < 'de > > > :: deserialize (deserializer) . map (| opt | opt . map (| wrap | wrap . 0)) }
};
}
