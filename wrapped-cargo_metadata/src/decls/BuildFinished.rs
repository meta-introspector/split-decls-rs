macro_rules! BuildFinished {
    () => {
        # [doc = " Final result of a build."] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] pub struct BuildFinished { # [doc = " Whether or not the build finished successfully."] pub success : bool , }
    };
}

BuildFinished!()