// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [wasm_bindgen (start)] fn run () -> Result < () , JsValue > { let f = Rc :: new (RefCell :: new (None)) ; let g = f . clone () ; let mut i = 0 ; * g . borrow_mut () = Some (Closure :: new (move | | { if i > 300 { body () . set_text_content (Some ("All done!")) ; let _ = f . borrow_mut () . take () ; return ; } i += 1 ; let text = format ! ("requestAnimationFrame has been called {i} times.") ; body () . set_text_content (Some (& text)) ; request_animation_frame (f . borrow () . as_ref () . unwrap ()) ; })) ; request_animation_frame (g . borrow () . as_ref () . unwrap ()) ; Ok (()) }
};
}
