// Generated macro for ResultExt (trait)
macro_rules! DepcrateResultExt {
() => {
// Module: crate
// Provides: {"ResultExt"}
// Dependencies: {}
# [doc = " Result extension trait adding a `context` method"] pub trait ResultExt < T , E > { # [doc = " The method is use to add context information to current operation"] # [doc = ""] # [doc = " The context data is then used in error constructor to store additional"] # [doc = " information within error. For example, you may add a filename as a"] # [doc = " context for file operation. See crate documentation for the actual"] # [doc = " example."] fn context < X > (self , x : X) -> Result < T , Context < X , E > > ; }
};
}
