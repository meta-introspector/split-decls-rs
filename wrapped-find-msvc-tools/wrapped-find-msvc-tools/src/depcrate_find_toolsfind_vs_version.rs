// Generated macro for find_vs_version (function)
macro_rules! Depcrate_find_toolsfind_vs_version {
() => {
// Module: crate::find_tools
// Provides: {"find_vs_version"}
// Dependencies: {}
# [doc = " Find the most recent installed version of Visual Studio"] # [doc = ""] # [doc = " This is used by the cmake crate to figure out the correct"] # [doc = " generator."] # [allow (clippy :: disallowed_methods)] pub fn find_vs_version () -> Result < VsVers , String > { fn has_msbuild_version (version : & str) -> bool { impl_ :: has_msbuild_version (version , & StdEnvGetter) } match std :: env :: var ("VisualStudioVersion") { Ok (version) => match & version [..] { "18.0" => Ok (VsVers :: Vs18) , "17.0" => Ok (VsVers :: Vs17) , "16.0" => Ok (VsVers :: Vs16) , "15.0" => Ok (VsVers :: Vs15) , "14.0" => Ok (VsVers :: Vs14) , vers => Err (format ! ("\n\n\
                 unsupported or unknown VisualStudio version: {vers}\n\
                 if another version is installed consider running \
                 the appropriate vcvars script before building this \
                 crate\n\
                 ")) , } , _ => { if has_msbuild_version ("18.0") { Ok (VsVers :: Vs18) } else if has_msbuild_version ("17.0") { Ok (VsVers :: Vs17) } else if has_msbuild_version ("16.0") { Ok (VsVers :: Vs16) } else if has_msbuild_version ("15.0") { Ok (VsVers :: Vs15) } else if has_msbuild_version ("14.0") { Ok (VsVers :: Vs14) } else { Err ("\n\n\
                     couldn't determine visual studio generator\n\
                     if VisualStudio is installed, however, consider \
                     running the appropriate vcvars script before building \
                     this crate\n\
                     " . to_string ()) } } } }
};
}
