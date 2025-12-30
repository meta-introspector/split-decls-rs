// Generated macro for Stream (trait)
macro_rules! DepcrateStream {
() => {
// Module: crate
// Provides: {"Stream"}
// Dependencies: {}
trait Stream { type Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Self :: Item > > ; }
};
}
