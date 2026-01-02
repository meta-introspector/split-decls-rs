mkuse!{use std :: borrow :: Cow ;}
mkuse!{use rustc_session :: Session ;}
mkmod!{apple, { 
                getname!(apple);
                getsrc!(apple);
                getpath!(apple);
                get_deps!(apple);
                get_crates!(apple);
                mkinclude!(apple);
                 
            }}
mkmod!{archive, { 
                getname!(archive);
                getsrc!(archive);
                getpath!(archive);
                get_deps!(archive);
                get_crates!(archive);
                mkinclude!(archive);
                 
            }}
mkmod!{command, { 
                getname!(command);
                getsrc!(command);
                getpath!(command);
                get_deps!(command);
                get_crates!(command);
                mkinclude!(command);
                 
            }}
mkmod!{link, { 
                getname!(link);
                getsrc!(link);
                getpath!(link);
                get_deps!(link);
                get_crates!(link);
                mkinclude!(link);
                 
            }}
mkmod!{linker, { 
                getname!(linker);
                getsrc!(linker);
                getpath!(linker);
                get_deps!(linker);
                get_crates!(linker);
                mkinclude!(linker);
                 
            }}
mkmod!{lto, { 
                getname!(lto);
                getsrc!(lto);
                getpath!(lto);
                get_deps!(lto);
                get_crates!(lto);
                mkinclude!(lto);
                 
            }}
mkmod!{metadata, { 
                getname!(metadata);
                getsrc!(metadata);
                getpath!(metadata);
                get_deps!(metadata);
                get_crates!(metadata);
                mkinclude!(metadata);
                 
            }}
mkmod!{rpath, { 
                getname!(rpath);
                getsrc!(rpath);
                getpath!(rpath);
                get_deps!(rpath);
                get_crates!(rpath);
                mkinclude!(rpath);
                 
            }}
mkmod!{symbol_export, { 
                getname!(symbol_export);
                getsrc!(symbol_export);
                getpath!(symbol_export);
                get_deps!(symbol_export);
                get_crates!(symbol_export);
                mkinclude!(symbol_export);
                 
            }}
mkmod!{write, { 
                getname!(write);
                getsrc!(write);
                getpath!(write);
                get_deps!(write);
                get_crates!(write);
                mkinclude!(write);
                 
            }}

macro_rules! versioned_llvm_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function versioned_llvm_target in module {}", module_path!());
    };
}

mkfn!{
    versioned_llvm_target_introspect!();
    # [doc = " The target triple depends on the deployment target, and is required to"] # [doc = " enable features such as cross-language LTO, and for picking the right"] # [doc = " Mach-O commands."] # [doc = ""] # [doc = " Certain optimizations also depend on the deployment target."] pub fn versioned_llvm_target (sess : & Session) -> Cow < '_ , str > { if sess . target . is_like_darwin { apple :: add_version_to_llvm_target (& sess . target . llvm_target , sess . apple_deployment_target ()) . into () } else { Cow :: Borrowed (& sess . target . llvm_target) } }
}