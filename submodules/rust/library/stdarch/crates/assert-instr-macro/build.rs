mkuse!{use std :: env ;}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { let opt_level = env :: var ("OPT_LEVEL") . ok () . and_then (| s | s . parse () . ok ()) . unwrap_or (0) ; let profile = env :: var ("PROFILE") . unwrap_or_default () ; if profile == "release" || opt_level >= 2 { println ! ("cargo:rustc-cfg=optimized") ; } }
}