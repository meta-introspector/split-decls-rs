// Generated macro for Component (enum)
macro_rules! Depcrate_globComponent {
() => {
// Module: crate::glob
// Provides: {"Component"}
// Dependencies: {}
# [derive (Clone)] enum Component < 'a > { # [doc = " Parent directory."] ParentDir , # [doc = " A normal component."] Normal (& 'a str) , # [doc = " Normal component, compiled into a fragment."] Fragment (Fragment < 'a >) , # [doc = " `**` component, which keeps expanding."] StarStar , }
};
}
