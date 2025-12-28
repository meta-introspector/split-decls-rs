macro_rules! RustcToolInfo {
    () => {
        # [derive (Debug , Clone)] pub struct RustcToolInfo { pub invocation_method : String , pub rustc_path : Option < String > , pub cargo_path : Option < String > , pub target_triple : Option < String > , pub sysroot : Option < String > , }
    };
}

RustcToolInfo!()