macro_rules! TargetKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum TargetKind { Bin , # [doc = " Any kind of Cargo lib crate-type (dylib, rlib, proc-macro, ...)."] Lib { # [doc = " Is this target a proc-macro"] is_proc_macro : bool , } , Example , Test , Bench , # [doc = " Cargo calls this kind `custom-build`"] BuildScript , Other , }
    };
}

TargetKind!();