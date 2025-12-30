// Generated macro for convert_sync (function)
macro_rules! Depcrateconvert_sync {
() => {
// Module: crate
// Provides: {"convert_sync"}
// Dependencies: {}
fn convert_sync (input : & mut Item) -> TokenStream2 { match input { Item :: Impl (item) => { for inner in & mut item . items { if let ImplItem :: Fn (ref mut method) = inner { if method . sig . asyncness . is_some () { method . sig . asyncness = None ; } } } AsyncAwaitRemoval . remove_async_await (quote ! (# item)) } Item :: Trait (item) => { for inner in & mut item . items { if let TraitItem :: Fn (ref mut method) = inner { if method . sig . asyncness . is_some () { method . sig . asyncness = None ; } } } AsyncAwaitRemoval . remove_async_await (quote ! (# item)) } Item :: Fn (item) => { if item . sig . asyncness . is_some () { item . sig . asyncness = None ; } AsyncAwaitRemoval . remove_async_await (quote ! (# item)) } Item :: Static (item) => AsyncAwaitRemoval . remove_async_await (quote ! (# item)) , } }
};
}
