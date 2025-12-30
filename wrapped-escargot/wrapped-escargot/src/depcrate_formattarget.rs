// Generated macro for Target (struct)
macro_rules! Depcrate_formatTarget {
() => {
// Module: crate::format
// Provides: {"Target"}
// Dependencies: {}
# [doc = " A single target (lib, bin, example, ...) provided by a crate"] # [derive (Clone , PartialEq , Eq , Serialize , Deserialize , Debug)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] # [non_exhaustive] pub struct Target < 'a > { # [doc = " Name as given in the `Cargo.toml` or generated from the file name"] # [serde (borrow)] pub name : CowStr < 'a > , # [doc = " Kind of target (\"bin\", \"example\", \"test\", \"bench\", \"lib\")"] # [serde (borrow)] pub kind : Vec < CowStr < 'a > > , # [doc = " Almost the same as `kind`, except when an example is a library instead of an executable."] # [doc = " In that case `crate_types` contains things like `rlib` and `dylib` while `kind` is `example`"] # [serde (default)] # [serde (borrow)] pub crate_types : Vec < CowStr < 'a > > , # [doc = " Whether this is a doctest or not"] # [serde (default)] pub doctest : Option < bool > , # [doc = " Whether this is documentation or not"] # [serde (default)] pub doc : Option < bool > , # [doc = " Whether this is a test file"] # [serde (default)] pub test : bool , # [serde (default)] # [serde (rename = "required-features")] # [doc = " This target is built only if these features are enabled."] # [doc = " It doesn't apply to `lib` targets."] # [serde (borrow)] pub required_features : Vec < CowStr < 'a > > , # [doc = " Path to the main source file of the target"] # [serde (borrow)] pub src_path : CowPath < 'a > , # [doc = " Rust edition for this target"] # [serde (default = "edition_default")] # [serde (borrow)] pub edition : CowStr < 'a > , }
};
}
