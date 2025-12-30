// Generated macro for lit_search_pat (function)
macro_rules! Depcrate_check_proc_macrolit_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"lit_search_pat"}
// Dependencies: {}
# [doc = " Get the search patterns to use for the given literal"] fn lit_search_pat (lit : & LitKind) -> (Pat , Pat) { match lit { LitKind :: Str (_ , StrStyle :: Cooked) => (Pat :: Str ("\"") , Pat :: Str ("\"")) , LitKind :: Str (_ , StrStyle :: Raw (0)) => (Pat :: Str ("r") , Pat :: Str ("\"")) , LitKind :: Str (_ , StrStyle :: Raw (_)) => (Pat :: Str ("r#") , Pat :: Str ("#")) , LitKind :: ByteStr (_ , StrStyle :: Cooked) => (Pat :: Str ("b\"") , Pat :: Str ("\"")) , LitKind :: ByteStr (_ , StrStyle :: Raw (0)) => (Pat :: Str ("br\"") , Pat :: Str ("\"")) , LitKind :: ByteStr (_ , StrStyle :: Raw (_)) => (Pat :: Str ("br#\"") , Pat :: Str ("#")) , LitKind :: Byte (_) => (Pat :: Str ("b'") , Pat :: Str ("'")) , LitKind :: Char (_) => (Pat :: Str ("'") , Pat :: Str ("'")) , LitKind :: Int (_ , LitIntType :: Signed (IntTy :: Isize)) => (Pat :: Num , Pat :: Str ("isize")) , LitKind :: Int (_ , LitIntType :: Unsigned (UintTy :: Usize)) => (Pat :: Num , Pat :: Str ("usize")) , LitKind :: Int (..) => (Pat :: Num , Pat :: Num) , LitKind :: Float (..) => (Pat :: Num , Pat :: Str ("")) , LitKind :: Bool (true) => (Pat :: Str ("true") , Pat :: Str ("true")) , LitKind :: Bool (false) => (Pat :: Str ("false") , Pat :: Str ("false")) , _ => (Pat :: Str ("") , Pat :: Str ("")) , } }
};
}
