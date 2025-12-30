// Generated macro for WriteStyle (enum)
macro_rules! Depcrate_writerWriteStyle {
() => {
// Module: crate::writer
// Provides: {"WriteStyle"}
// Dependencies: {}
# [doc = " Whether or not to print styles to the target."] # [allow (clippy :: exhaustive_enums)] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , Default)] pub enum WriteStyle { # [doc = " Try to print styles, but don't force the issue."] # [default] Auto , # [doc = " Try very hard to print styles."] Always , # [doc = " Never print styles."] Never , }
};
}
