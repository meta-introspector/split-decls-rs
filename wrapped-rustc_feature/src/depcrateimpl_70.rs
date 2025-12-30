// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl UnstableFeatures { # [doc = " This takes into account `RUSTC_BOOTSTRAP`."] # [doc = ""] # [doc = " If `krate` is [`Some`], then setting `RUSTC_BOOTSTRAP=krate` will enable the nightly"] # [doc = " features. Otherwise, only `RUSTC_BOOTSTRAP=1` will work."] pub fn from_environment (krate : Option < & str >) -> Self { Self :: from_environment_value (krate , std :: env :: var ("RUSTC_BOOTSTRAP")) } # [doc = " Avoid unsafe `std::env::set_var()` by allowing tests to inject"] # [doc = " `std::env::var(\"RUSTC_BOOTSTRAP\")` with the `env_var_rustc_bootstrap`"] # [doc = " arg."] fn from_environment_value (krate : Option < & str > , env_var_rustc_bootstrap : Result < String , std :: env :: VarError > ,) -> Self { let disable_unstable_features = option_env ! ("CFG_DISABLE_UNSTABLE_FEATURES") . is_some_and (| s | s != "0") ; let is_unstable_crate = | var : & str | krate . is_some_and (| name | var . split (',') . any (| new_krate | new_krate == name)) ; let bootstrap = env_var_rustc_bootstrap . ok () ; if let Some (val) = bootstrap . as_deref () { match val { val if val == "1" || is_unstable_crate (val) => return UnstableFeatures :: Cheat , "-1" => return UnstableFeatures :: Disallow , _ => { } } } if disable_unstable_features { UnstableFeatures :: Disallow } else { UnstableFeatures :: Allow } } pub fn is_nightly_build (& self) -> bool { match * self { UnstableFeatures :: Allow | UnstableFeatures :: Cheat => true , UnstableFeatures :: Disallow => false , } } }
};
}
