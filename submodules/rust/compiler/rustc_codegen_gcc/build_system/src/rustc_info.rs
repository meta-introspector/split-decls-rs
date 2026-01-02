mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: utils :: run_command ;}

macro_rules! get_rustc_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_rustc_path in module {}", module_path!());
    };
}

mkfn!{
    get_rustc_path_introspect!();
    pub fn get_rustc_path () -> Option < PathBuf > { if let Ok (rustc) = std :: env :: var ("RUSTC") { return Some (PathBuf :: from (rustc)) ; } run_command (& [& "rustup" , & "which" , & "rustc"] , None) . ok () . map (| out | Path :: new (String :: from_utf8 (out . stdout) . unwrap () . trim ()) . to_path_buf ()) }
}