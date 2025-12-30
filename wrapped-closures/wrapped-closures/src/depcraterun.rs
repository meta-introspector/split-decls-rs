// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [wasm_bindgen (start)] fn run () -> Result < () , JsValue > { let window = web_sys :: window () . expect ("should have a window in this context") ; let document = window . document () . expect ("window should have a document") ; let array = Array :: new () ; array . push (& "Hello" . into ()) ; array . push (& 1 . into ()) ; let mut first_item = None ; array . for_each (& mut | obj , idx , _arr | match idx { 0 => { assert_eq ! (obj , "Hello") ; first_item = obj . as_string () ; } 1 => assert_eq ! (obj , 1) , _ => panic ! ("unknown index: {idx}") , }) ; assert_eq ! (first_item , Some ("Hello" . to_string ())) ; setup_clock (& window , & document) ? ; setup_clicker (& document) ; document . get_element_by_id ("loading") . expect ("should have #loading on the page") . dyn_ref :: < HtmlElement > () . expect ("#loading should be an `HtmlElement`") . style () . set_property ("display" , "none") ? ; document . get_element_by_id ("script") . expect ("should have #script on the page") . dyn_ref :: < HtmlElement > () . expect ("#script should be an `HtmlElement`") . style () . set_property ("display" , "block") ? ; Ok (()) }
};
}
