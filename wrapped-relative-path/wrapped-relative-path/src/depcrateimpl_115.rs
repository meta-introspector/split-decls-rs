// Generated macro for impl_115 (impl)
macro_rules! Depcrateimpl_115 {
() => {
// Module: crate
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'a > Component < 'a > { # [doc = " Extracts the underlying [`str`] slice."] # [doc = ""] # [doc = " [`str`]: prim@str"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::{RelativePath, Component};"] # [doc = ""] # [doc = " let path = RelativePath::new(\"./tmp/../foo/bar.txt\");"] # [doc = " let components: Vec<_> = path.components().map(Component::as_str).collect();"] # [doc = " assert_eq!(&components, &[\".\", \"tmp\", \"..\", \"foo\", \"bar.txt\"]);"] # [doc = " ```"] # [must_use] # [inline] pub fn as_str (self) -> & 'a str { use self :: Component :: { CurDir , Normal , ParentDir } ; match self { CurDir => CURRENT_STR , ParentDir => PARENT_STR , Normal (name) => name , } } }
};
}
