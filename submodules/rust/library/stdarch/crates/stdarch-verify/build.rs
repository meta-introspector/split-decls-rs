mkuse!{use std :: path :: Path ;}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { let dir = Path :: new (env ! ("CARGO_MANIFEST_DIR")) ; let root = dir . parent () . unwrap () ; eprintln ! ("root: {}" , root . display ()) ; walk (& root . join ("core_arch/src/x86")) ; walk (& root . join ("core_arch/src/x86_64")) ; walk (& root . join ("core_arch/src/arm")) ; walk (& root . join ("core_arch/src/aarch64")) ; }
}

macro_rules! walk_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function walk in module {}", module_path!());
    };
}

mkfn!{
    walk_introspect!();
    fn walk (root : & Path) { for file in root . read_dir () . unwrap () { eprintln ! ("root: {}" , root . display ()) ; let file = file . unwrap () ; if file . file_type () . unwrap () . is_dir () { walk (& file . path ()) ; continue ; } let path = file . path () ; if path . extension () . and_then (| s | s . to_str ()) != Some ("rs") { continue ; } println ! ("cargo:rerun-if-changed={}" , path . display ()) ; } }
}