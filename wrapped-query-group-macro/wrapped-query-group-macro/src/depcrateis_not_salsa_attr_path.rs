// Generated macro for is_not_salsa_attr_path (function)
macro_rules! Depcrateis_not_salsa_attr_path {
() => {
// Module: crate
// Provides: {"is_not_salsa_attr_path"}
// Dependencies: {}
fn is_not_salsa_attr_path (path : & syn :: Path) -> bool { path . segments . first () . map (| s | s . ident != "salsa") . unwrap_or (true) || path . segments . len () != 2 }
};
}
