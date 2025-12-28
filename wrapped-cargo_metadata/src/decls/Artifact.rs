macro_rules! deps {
    () => {
        ArtifactProfile!();
        Target!();
        PackageId!();
    };
}

macro_rules! Artifact {
    () => {
        deps!();
        # [doc = " A compiler-generated file."] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] pub struct Artifact { # [doc = " The package this artifact belongs to"] pub package_id : PackageId , # [doc = " Path to the `Cargo.toml` file"] # [serde (default)] pub manifest_path : Utf8PathBuf , # [doc = " The target this artifact was compiled for"] pub target : Target , # [doc = " The profile this artifact was compiled with"] pub profile : ArtifactProfile , # [doc = " The enabled features for this artifact"] pub features : Vec < String > , # [doc = " The full paths to the generated artifacts"] # [doc = " (e.g. binary file and separate debug info)"] pub filenames : Vec < Utf8PathBuf > , # [doc = " Path to the executable file"] pub executable : Option < Utf8PathBuf > , # [doc = " If true, then the files were already generated"] pub fresh : bool , }
    };
}

Artifact!()