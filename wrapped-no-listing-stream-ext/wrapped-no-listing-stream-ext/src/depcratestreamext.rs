// Generated macro for StreamExt (trait)
macro_rules! DepcrateStreamExt {
() => {
// Module: crate
// Provides: {"StreamExt"}
// Dependencies: {}
trait StreamExt : Stream { async fn next (& mut self) -> Option < Self :: Item > where Self : Unpin ; }
};
}
