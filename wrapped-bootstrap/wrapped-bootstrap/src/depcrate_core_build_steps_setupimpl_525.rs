// Generated macro for impl_525 (impl)
macro_rules! Depcrate_core_build_steps_setupimpl_525 {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"impl_525"}
// Dependencies: {}
impl Profile { fn include_path (& self , src_path : & Path) -> PathBuf { PathBuf :: from (format ! ("{}/{PROFILE_DIR}/bootstrap.{}.toml" , src_path . display () , self)) } pub fn all () -> impl Iterator < Item = Self > { use Profile :: * ; [Library , Compiler , Tools , Dist , None] . iter () . copied () } pub fn purpose (& self) -> String { use Profile :: * ; match self { Library => "Contribute to the standard library" , Compiler => "Contribute to the compiler itself" , Tools => "Contribute to tools which depend on the compiler, but do not modify it directly (e.g. rustdoc, clippy, miri)" , Dist => "Install Rust from source" , None => "Do not modify `bootstrap.toml`" } . to_string () } pub fn all_for_help (indent : & str) -> String { let mut out = String :: new () ; for choice in Profile :: all () { writeln ! (& mut out , "{}{}: {}" , indent , choice , choice . purpose ()) . unwrap () ; } out } pub fn as_str (& self) -> & 'static str { match self { Profile :: Compiler => "compiler" , Profile :: Library => "library" , Profile :: Tools => "tools" , Profile :: Dist => "dist" , Profile :: None => "none" , } } }
};
}
