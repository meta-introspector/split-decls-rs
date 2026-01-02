mkuse!{use std :: path :: PathBuf ;}
mkuse!{use std :: sync :: OnceLock ;}
mkuse!{use rustc_data_structures :: profiling :: VerboseTimingGuard ;}
mkuse!{use rustc_fs_util :: try_canonicalize ;}
mkuse!{use rustc_hir :: attrs :: NativeLibKind ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable_Generic } ;}
mkuse!{use crate :: session :: Session ;}
mkitem!{mkimpl!{impl Session { pub fn timer (& self , what : & 'static str) -> VerboseTimingGuard < '_ > { self . prof . verbose_generic_activity (what) } # [doc = " Used by `-Z self-profile`."] pub fn time < R > (& self , what : & 'static str , f : impl FnOnce () -> R) -> R { self . prof . verbose_generic_activity (what) . run (f) } }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Encodable , Decodable)] # [derive (HashStable_Generic)] pub struct NativeLib { pub name : String , pub new_name : Option < String > , pub kind : NativeLibKind , pub verbatim : Option < bool > , }}}
mkitem!{mkimpl!{impl NativeLib { pub fn has_modifiers (& self) -> bool { self . verbatim . is_some () || self . kind . has_modifiers () } }}}
mkitem!{mkstruct!{# [doc = " A path that has been canonicalized along with its original, non-canonicalized form"] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub struct CanonicalizedPath { canonicalized : Option < PathBuf > , original : PathBuf , }}}
mkitem!{mkimpl!{impl CanonicalizedPath { pub fn new (path : PathBuf) -> Self { Self { canonicalized : try_canonicalize (& path) . ok () , original : path } } pub fn canonicalized (& self) -> & PathBuf { self . canonicalized . as_ref () . unwrap_or (self . original ()) } pub fn original (& self) -> & PathBuf { & self . original } }}}

macro_rules! extra_compiler_flags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extra_compiler_flags in module {}", module_path!());
    };
}

mkfn!{
    extra_compiler_flags_introspect!();
    # [doc = " Gets a list of extra command-line flags provided by the user, as strings."] # [doc = ""] # [doc = " This function is used during ICEs to show more information useful for"] # [doc = " debugging, since some ICEs only happens with non-default compiler flags"] # [doc = " (and the users don't always report them)."] pub fn extra_compiler_flags () -> Option < (Vec < String > , bool) > { const ICE_REPORT_COMPILER_FLAGS : & [& str] = & ["-Z" , "-C" , "--crate-type"] ; const ICE_REPORT_COMPILER_FLAGS_EXCLUDE : & [& str] = & ["metadata" , "extra-filename"] ; const ICE_REPORT_COMPILER_FLAGS_STRIP_VALUE : & [& str] = & ["incremental"] ; let mut args = std :: env :: args_os () . map (| arg | arg . to_string_lossy () . to_string ()) ; let mut result = Vec :: new () ; let mut excluded_cargo_defaults = false ; while let Some (arg) = args . next () { if let Some (a) = ICE_REPORT_COMPILER_FLAGS . iter () . find (| a | arg . starts_with (* a)) { let content = if arg . len () == a . len () { match args . next () { Some (arg) => arg , None => continue , } } else if arg . get (a . len () .. a . len () + 1) == Some ("=") { arg [a . len () + 1 ..] . to_string () } else { arg [a . len () ..] . to_string () } ; let option = content . split_once ('=') . map (| s | s . 0) . unwrap_or (& content) ; if ICE_REPORT_COMPILER_FLAGS_EXCLUDE . contains (& option) { excluded_cargo_defaults = true ; } else { result . push (a . to_string ()) ; result . push (if ICE_REPORT_COMPILER_FLAGS_STRIP_VALUE . contains (& option) { format ! ("{option}=[REDACTED]") } else { content }) ; } } } if ! result . is_empty () { Some ((result , excluded_cargo_defaults)) } else { None } }
}

macro_rules! was_invoked_from_cargo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function was_invoked_from_cargo in module {}", module_path!());
    };
}

mkfn!{
    was_invoked_from_cargo_introspect!();
    # [doc = " Returns whenever rustc was launched by Cargo as opposed to another build system."] # [doc = ""] # [doc = " To be used in diagnostics to avoid printing Cargo specific suggestions to other"] # [doc = " build systems (like Bazel, Buck2, Makefile, ...)."] pub fn was_invoked_from_cargo () -> bool { static FROM_CARGO : OnceLock < bool > = OnceLock :: new () ; * FROM_CARGO . get_or_init (| | std :: env :: var_os ("CARGO_CRATE_NAME") . is_some ()) }
}