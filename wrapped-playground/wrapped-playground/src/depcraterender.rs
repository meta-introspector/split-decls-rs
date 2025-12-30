// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
# [wasm_bindgen] pub fn render (template_str : & str , data : JsValue) -> Result < String , String > { let hbs = Handlebars :: new () ; hbs . render_template (template_str , & data . into_serde :: < Value > () . unwrap ()) . map_err (| e | format ! ("{}" , e)) }
};
}
