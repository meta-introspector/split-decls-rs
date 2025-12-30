// Generated macro for impl_87 (impl)
macro_rules! Depcrateimpl_87 {
() => {
// Module: crate
// Provides: {"impl_87"}
// Dependencies: {}
impl fmt :: Debug for PrettyPrintRegistry { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "vec![") ? ; for s in & self . 0 { if s . dependencies () . is_empty () { write ! (f , "pkg!((\"{}\", \"{}\"))," , s . name () , s . version ()) ? ; } else { write ! (f , "pkg!((\"{}\", \"{}\") => [" , s . name () , s . version ()) ? ; for d in s . dependencies () { if d . kind () == DepKind :: Normal && & d . version_req () . to_string () == "*" && ! d . is_public () { write ! (f , "dep(\"{}\")," , d . name_in_toml ()) ? ; } else if d . kind () == DepKind :: Normal && ! d . is_public () { write ! (f , "dep_req(\"{}\", \"{}\")," , d . name_in_toml () , d . version_req ()) ? ; } else { write ! (f , "dep_req_kind(\"{}\", \"{}\", {}, {})," , d . name_in_toml () , d . version_req () , match d . kind () { DepKind :: Development => "DepKind::Development" , DepKind :: Build => "DepKind::Build" , DepKind :: Normal => "DepKind::Normal" , } , d . is_public ()) ? ; } } write ! (f , "]),") ? ; } } write ! (f , "]") } }
};
}
