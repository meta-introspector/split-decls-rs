macro_rules! deps {
    () => {
        Build!();
        Dep!();
    };
}

macro_rules! Crate {
    () => {
        deps!();
        # [doc = " A crate points to the root module of a crate and lists the dependencies of the crate. This is"] # [doc = " useful in creating the crate graph."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Crate { pub (crate) display_name : Option < CrateDisplayName > , pub root_module : AbsPathBuf , pub (crate) edition : Edition , pub (crate) version : Option < String > , pub (crate) deps : Vec < Dep > , pub (crate) cfg : Vec < CfgAtom > , pub (crate) target : Option < String > , pub (crate) env : FxHashMap < String , String > , pub (crate) proc_macro_dylib_path : Option < AbsPathBuf > , pub (crate) is_workspace_member : bool , pub (crate) include : Vec < AbsPathBuf > , pub (crate) exclude : Vec < AbsPathBuf > , pub (crate) is_proc_macro : bool , # [doc = " The working directory to run proc-macros in. This is usually the workspace root of cargo workspaces."] pub (crate) proc_macro_cwd : Option < AbsPathBuf > , pub (crate) repository : Option < String > , pub build : Option < Build > , }
    };
}

Crate!();