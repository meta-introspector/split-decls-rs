// Generated macro for Artifact (struct)
macro_rules! Depcrate_formatArtifact {
() => {
// Module: crate::format
// Provides: {"Artifact"}
// Dependencies: {}
# [doc = " A compiler-generated file."] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] # [non_exhaustive] pub struct Artifact < 'a > { # [doc = " The workspace member this artifact belongs to"] # [serde (borrow)] pub package_id : WorkspaceMember < 'a > , # [doc = " The full path to the artifact's manifest"] # [serde (borrow)] pub manifest_path : Option < CowPath < 'a > > , # [doc = " The cargo target (lib, bin, example, etc.) that generated the artifacts."] # [serde (borrow)] pub target : Target < 'a > , # [doc = " The profile indicates which compiler settings were used."] # [serde (borrow)] pub profile : ArtifactProfile < 'a > , # [doc = " The enabled features for this artifact"] # [serde (borrow)] pub features : Vec < CowStr < 'a > > , # [doc = " The full paths to the generated artifacts"] # [serde (borrow)] pub filenames : Vec < CowPath < 'a > > , # [doc = " The path to the executable that was created"] # [doc = ""] # [doc = " `None` if this step did not generate an executable."] # [serde (borrow)] # [serde (default)] pub executable : Option < CowPath < 'a > > , # [doc = " Whether or not this step was actually executed."] # [doc = ""] # [doc = " When `true`, this means that the pre-existing artifacts were"] # [doc = " up-to-date, and `rustc` was not executed. When `false`, this means that"] # [doc = " `rustc` was run to generate the artifacts."] pub fresh : bool , }
};
}
