// Generated macro for from_id (macro)
macro_rules! Depcrate_from_idfrom_id {
() => {
// Module: crate::from_id
// Provides: {"from_id"}
// Dependencies: {}
macro_rules ! from_id { ($ (($ id : path , $ ty : path)) ,* $ (,) ?) => { $ (impl From <$ id > for $ ty { fn from (id : $ id) -> $ ty { $ ty { id } } } impl From <$ ty > for $ id { fn from (ty : $ ty) -> $ id { ty . id } }) * } }
};
}
