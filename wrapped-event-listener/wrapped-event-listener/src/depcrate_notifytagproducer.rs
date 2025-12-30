// Generated macro for TagProducer (trait)
macro_rules! Depcrate_notifyTagProducer {
() => {
// Module: crate::notify
// Provides: {"TagProducer"}
// Dependencies: {}
# [doc = " The producer for a generic notification."] pub (crate) trait TagProducer { type Tag ; # [doc = " Get the next tag."] fn next_tag (& mut self) -> Self :: Tag ; }
};
}
