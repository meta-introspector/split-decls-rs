// Generated macro for impl_103 (impl)
macro_rules! Depcrateimpl_103 {
() => {
// Module: crate
// Provides: {"impl_103"}
// Dependencies: {}
impl PathExt for Path { # [inline] fn expand (& self) -> io :: Result < Cow < '_ , Self > > { imp :: expand (self) } # [cfg (feature = "localization")] # [inline] fn localize_name (& self) -> Cow < '_ , OsStr > { let Some (name) = self . components () . next_back () else { return Cow :: Borrowed (OsStr :: new ("")) ; } ; assert_ne ! (Component :: ParentDir , name , "path ends with a `..` component: \"{}\"" , self . display () ,) ; localize :: name (self) . map (Cow :: Owned) . unwrap_or_else (| | Cow :: Borrowed (name . as_os_str ())) } # [inline] fn normalize (& self) -> io :: Result < BasePathBuf > { imp :: normalize (self) } # [cfg (any (doc , windows))] # [inline] fn normalize_virtually (& self) -> io :: Result < BasePathBuf > { imp :: normalize_virtually (self) } # [inline] fn shorten (& self) -> io :: Result < Cow < '_ , Self > > { imp :: shorten (self) } }
};
}
