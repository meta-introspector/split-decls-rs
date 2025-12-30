// Generated macro for text_of (function)
macro_rules! Depcrate_figuretext_of {
() => {
// Module: crate::figure
// Provides: {"text_of"}
// Dependencies: {}
fn text_of (node : Node) -> Option < String > { match node { Node :: Text (text) => Some (text) , Node :: Element (element) => { Some (element . children . into_iter () . filter_map (text_of) . collect ()) } Node :: Comment (_) => None , } }
};
}
