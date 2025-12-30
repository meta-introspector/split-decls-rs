// Generated macro for should_refresh_for_change (function)
macro_rules! Depcrate_reloadshould_refresh_for_change {
() => {
// Module: crate::reload
// Provides: {"should_refresh_for_change"}
// Dependencies: {}
pub (crate) fn should_refresh_for_change (path : & AbsPath , change_kind : ChangeKind , additional_paths : & [& str] ,) -> bool { const IMPLICIT_TARGET_FILES : & [& str] = & ["build.rs" , "src/main.rs" , "src/lib.rs"] ; const IMPLICIT_TARGET_DIRS : & [& str] = & ["src/bin" , "examples" , "tests" , "benches"] ; let file_name = match path . file_name () { Some (it) => it , None => return false , } ; if let "Cargo.toml" | "Cargo.lock" = file_name { return true ; } if additional_paths . contains (& file_name) { return true ; } if change_kind == ChangeKind :: Modify { return false ; } if path . extension () . unwrap_or_default () != "rs" { let is_cargo_config = matches ! (file_name , "config.toml" | "config") && path . parent () . map (| parent | parent . as_str () . ends_with (".cargo")) . unwrap_or (false) ; return is_cargo_config ; } if IMPLICIT_TARGET_FILES . iter () . any (| it | path . as_str () . ends_with (it)) { return true ; } let parent = match path . parent () { Some (it) => it , None => return false , } ; if IMPLICIT_TARGET_DIRS . iter () . any (| it | parent . as_str () . ends_with (it)) { return true ; } if file_name == "main.rs" { let grand_parent = match parent . parent () { Some (it) => it , None => return false , } ; if IMPLICIT_TARGET_DIRS . iter () . any (| it | grand_parent . as_str () . ends_with (it)) { return true ; } } false }
};
}
