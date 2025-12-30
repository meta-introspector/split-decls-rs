// Generated macro for gen_bindgen_attr (macro)
macro_rules! Depcrate_parsergen_bindgen_attr {
() => {
// Module: crate::parser
// Provides: {"gen_bindgen_attr"}
// Dependencies: {}
macro_rules ! gen_bindgen_attr { ($ (($ method : ident , $ _ : literal , $ ($ variants : tt) *) ,) *) => { # [doc = " The possible attributes in the `#[wasm_bindgen]`."] # [cfg_attr (feature = "extra-traits" , derive (Debug))] pub enum BindgenAttr { $ ($ ($ variants) *,) * } } }
};
}
