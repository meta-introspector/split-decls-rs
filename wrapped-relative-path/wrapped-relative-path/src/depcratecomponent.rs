// Generated macro for Component (enum)
macro_rules! DepcrateComponent {
() => {
// Module: crate
// Provides: {"Component"}
// Dependencies: {}
# [doc = " A single path component."] # [doc = ""] # [doc = " Accessed using the [`RelativePath::components`] iterator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::{Component, RelativePath};"] # [doc = ""] # [doc = " let path = RelativePath::new(\"foo/../bar/./baz\");"] # [doc = " let mut it = path.components();"] # [doc = ""] # [doc = " assert_eq!(Some(Component::Normal(\"foo\")), it.next());"] # [doc = " assert_eq!(Some(Component::ParentDir), it.next());"] # [doc = " assert_eq!(Some(Component::Normal(\"bar\")), it.next());"] # [doc = " assert_eq!(Some(Component::CurDir), it.next());"] # [doc = " assert_eq!(Some(Component::Normal(\"baz\")), it.next());"] # [doc = " assert_eq!(None, it.next());"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq , PartialOrd , Ord)] pub enum Component < 'a > { # [doc = " The current directory `.`."] CurDir , # [doc = " The parent directory `..`."] ParentDir , # [doc = " A normal path component as a string."] Normal (& 'a str) , }
};
}
