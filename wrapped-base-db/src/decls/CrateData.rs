macro_rules! deps {
    () => {
        Dependency!();
        CrateOrigin!();
    };
}

macro_rules! CrateData {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct CrateData < Id > { pub root_file_id : FileId , pub edition : Edition , # [doc = " The dependencies of this crate."] # [doc = ""] # [doc = " Note that this may contain more dependencies than the crate actually uses."] # [doc = " A common example is the test crate which is included but only actually is active when"] # [doc = " declared in source via `extern crate test`."] pub dependencies : Vec < Dependency < Id > > , pub origin : CrateOrigin , pub is_proc_macro : bool , # [doc = " The working directory to run proc-macros in invoked in the context of this crate."] # [doc = " This is the workspace root of the cargo workspace for workspace members, the crate manifest"] # [doc = " dir otherwise."] pub proc_macro_cwd : Arc < AbsPathBuf > , }
    };
}

CrateData!();