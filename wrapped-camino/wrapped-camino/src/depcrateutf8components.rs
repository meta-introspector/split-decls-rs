// Generated macro for Utf8Components (struct)
macro_rules! DepcrateUtf8Components {
() => {
// Module: crate
// Provides: {"Utf8Components"}
// Dependencies: {}
# [doc = " An iterator over the [`Utf8Component`]s of a [`Utf8Path`]."] # [doc = ""] # [doc = " This `struct` is created by the [`components`] method on [`Utf8Path`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use camino::Utf8Path;"] # [doc = ""] # [doc = " let path = Utf8Path::new(\"/tmp/foo/bar.txt\");"] # [doc = ""] # [doc = " for component in path.components() {"] # [doc = "     println!(\"{:?}\", component);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`components`]: Utf8Path::components"] # [derive (Clone , Eq , Ord , PartialEq , PartialOrd)] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct Utf8Components < 'a > (Components < 'a >) ;
};
}
