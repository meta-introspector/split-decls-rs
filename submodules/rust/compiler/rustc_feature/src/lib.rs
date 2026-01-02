mkmod!{accepted, { 
                getname!(accepted);
                getsrc!(accepted);
                getpath!(accepted);
                get_deps!(accepted);
                get_crates!(accepted);
                mkinclude!(accepted);
                 
            }}
mkmod!{builtin_attrs, { 
                getname!(builtin_attrs);
                getsrc!(builtin_attrs);
                getpath!(builtin_attrs);
                get_deps!(builtin_attrs);
                get_crates!(builtin_attrs);
                mkinclude!(builtin_attrs);
                 
            }}
mkmod!{removed, { 
                getname!(removed);
                getsrc!(removed);
                getpath!(removed);
                get_deps!(removed);
                get_crates!(removed);
                mkinclude!(removed);
                 
            }}
mkmod!{unstable, { 
                getname!(unstable);
                getsrc!(unstable);
                getpath!(unstable);
                get_deps!(unstable);
                get_crates!(unstable);
                mkinclude!(unstable);
                 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use std :: num :: NonZero ;}
mkuse!{use rustc_span :: Symbol ;}
mkitem!{mkstruct!{# [derive (Debug , Clone)] pub struct Feature { pub name : Symbol , # [doc = " For unstable features: the version the feature was added in."] # [doc = " For accepted features: the version the feature got stabilized in."] # [doc = " For removed features we are inconsistent; sometimes this is the"] # [doc = " version it got added, sometimes the version it got removed."] pub since : & 'static str , issue : Option < NonZero < u32 > > , }}}
mkitem!{mkenum!{# [derive (Clone , Copy , Debug , Hash)] pub enum UnstableFeatures { # [doc = " Disallow use of unstable features, as on beta/stable channels."] Disallow , # [doc = " Allow use of unstable features, as on nightly."] Allow , # [doc = " Errors are bypassed for bootstrapping. This is required any time"] # [doc = " during the build that feature-related lints are set to warn or above"] # [doc = " because the build turns on warnings-as-errors and uses lots of unstable"] # [doc = " features. As a result, this is always required for building Rust itself."] Cheat , }}}
mkitem!{mkimpl!{impl UnstableFeatures { # [doc = " This takes into account `RUSTC_BOOTSTRAP`."] # [doc = ""] # [doc = " If `krate` is [`Some`], then setting `RUSTC_BOOTSTRAP=krate` will enable the nightly"] # [doc = " features. Otherwise, only `RUSTC_BOOTSTRAP=1` will work."] pub fn from_environment (krate : Option < & str >) -> Self { Self :: from_environment_value (krate , std :: env :: var ("RUSTC_BOOTSTRAP")) } # [doc = " Avoid unsafe `std::env::set_var()` by allowing tests to inject"] # [doc = " `std::env::var(\"RUSTC_BOOTSTRAP\")` with the `env_var_rustc_bootstrap`"] # [doc = " arg."] fn from_environment_value (krate : Option < & str > , env_var_rustc_bootstrap : Result < String , std :: env :: VarError > ,) -> Self { let disable_unstable_features = option_env ! ("CFG_DISABLE_UNSTABLE_FEATURES") . is_some_and (| s | s != "0") ; let is_unstable_crate = | var : & str | krate . is_some_and (| name | var . split (',') . any (| new_krate | new_krate == name)) ; let bootstrap = env_var_rustc_bootstrap . ok () ; if let Some (val) = bootstrap . as_deref () { match val { val if val == "1" || is_unstable_crate (val) => return UnstableFeatures :: Cheat , "-1" => return UnstableFeatures :: Disallow , _ => { } } } if disable_unstable_features { UnstableFeatures :: Disallow } else { UnstableFeatures :: Allow } } pub fn is_nightly_build (& self) -> bool { match * self { UnstableFeatures :: Allow | UnstableFeatures :: Cheat => true , UnstableFeatures :: Disallow => false , } } }}}

macro_rules! find_lang_feature_issue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_lang_feature_issue in module {}", module_path!());
    };
}

mkfn!{
    find_lang_feature_issue_introspect!();
    fn find_lang_feature_issue (feature : Symbol) -> Option < NonZero < u32 > > { if let Some (f) = UNSTABLE_LANG_FEATURES . iter () . find (| f | f . name == feature) { return f . issue ; } if let Some (f) = ACCEPTED_LANG_FEATURES . iter () . find (| f | f . name == feature) { return f . issue ; } if let Some (f) = REMOVED_LANG_FEATURES . iter () . find (| f | f . feature . name == feature) { return f . feature . issue ; } panic ! ("feature `{feature}` is not declared anywhere") ; }
}

macro_rules! to_nonzero_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_nonzero in module {}", module_path!());
    };
}

mkfn!{
    to_nonzero_introspect!();
    const fn to_nonzero (n : Option < u32 >) -> Option < NonZero < u32 > > { match n { None => None , Some (n) => NonZero :: new (n) , } }
}
mkitem!{mkenum!{pub enum GateIssue { Language , Library (Option < NonZero < u32 > >) , }}}

macro_rules! find_feature_issue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_feature_issue in module {}", module_path!());
    };
}

mkfn!{
    find_feature_issue_introspect!();
    pub fn find_feature_issue (feature : Symbol , issue : GateIssue) -> Option < NonZero < u32 > > { match issue { GateIssue :: Language => find_lang_feature_issue (feature) , GateIssue :: Library (lib) => lib , } }
}
mkuse!{pub use accepted :: ACCEPTED_LANG_FEATURES ;}
mkuse!{pub use builtin_attrs :: { AttributeDuplicates , AttributeGate , AttributeSafety , AttributeTemplate , AttributeType , BUILTIN_ATTRIBUTE_MAP , BUILTIN_ATTRIBUTES , BuiltinAttribute , GatedCfg , encode_cross_crate , find_gated_cfg , is_builtin_attr_name , is_stable_diagnostic_attribute , is_valid_for_get_attr , } ;}
mkuse!{pub use removed :: REMOVED_LANG_FEATURES ;}
mkuse!{pub use unstable :: { EnabledLangFeature , EnabledLibFeature , Features , INCOMPATIBLE_FEATURES , UNSTABLE_LANG_FEATURES , } ;}