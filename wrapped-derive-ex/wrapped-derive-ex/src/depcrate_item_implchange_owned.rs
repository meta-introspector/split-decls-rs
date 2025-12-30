// Generated macro for change_owned (function)
macro_rules! Depcrate_item_implchange_owned {
() => {
// Module: crate::item_impl
// Provides: {"change_owned"}
// Dependencies: {}
fn change_owned (expr : TokenStream , ty : & Type , input_ref : bool , output_ref : bool) -> TokenStream { match (input_ref , output_ref) { (true , false) => quote ! (<# ty as :: core :: clone :: Clone >:: clone (# expr)) , (false , true) => quote ! (&# expr) , _ => expr , } }
};
}
