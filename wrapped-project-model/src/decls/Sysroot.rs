macro_rules! deps {
    () => {
        RustLibSrcWorkspace!();
    };
}

macro_rules! Sysroot {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct Sysroot { root : Option < AbsPathBuf > , rust_lib_src_root : Option < AbsPathBuf > , workspace : RustLibSrcWorkspace , error : Option < String > , }
    };
}

Sysroot!()