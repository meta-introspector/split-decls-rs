mkuse!{use std :: env ;}

macro_rules! main_introspect {
    () => {
        println!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkitem!{mkfn!{
    main_introspect!();
    fn main () { let target_os = env :: var ("CARGO_CFG_TARGET_OS") ; let target_env = env :: var ("CARGO_CFG_TARGET_ENV") ; if Ok ("windows") == target_os . as_deref () && Ok ("msvc") == target_env . as_deref () { set_windows_exe_options () ; } else { println ! ("cargo:rerun-if-changed=build.rs") ; } }
}}

macro_rules! set_windows_exe_options_introspect {
    () => {
        println!("📊 INTROSPECT: Function set_windows_exe_options in module {}", module_path!());
    };
}

mkitem!{mkfn!{
    set_windows_exe_options_introspect!();
    fn set_windows_exe_options () { static WINDOWS_MANIFEST_FILE : & str = "Windows Manifest.xml" ; let mut manifest = env :: current_dir () . unwrap () ; manifest . push (WINDOWS_MANIFEST_FILE) ; println ! ("cargo:rerun-if-changed={WINDOWS_MANIFEST_FILE}") ; println ! ("cargo:rustc-link-arg-bin=rustc-main=/MANIFEST:EMBED") ; println ! ("cargo:rustc-link-arg-bin=rustc-main=/MANIFESTINPUT:{}" , manifest . to_str () . unwrap ()) ; println ! ("cargo:rustc-link-arg-bin=rustc-main=/WX") ; }
}}