macro_rules! ArtifactDebuginfo {
    () => {
        # [doc = " The kind of debug information included in the artifact."] # [derive (Debug , Clone , PartialEq , Eq , Hash , Default)] # [non_exhaustive] pub enum ArtifactDebuginfo { # [doc = " No debug information."] # [default] None , # [doc = " Line directives only."] LineDirectivesOnly , # [doc = " Line tables only."] LineTablesOnly , # [doc = " Debug information without type or variable-level information."] Limited , # [doc = " Full debug information."] Full , # [doc = " An unknown integer level."] # [doc = ""] # [doc = " This may be produced by a version of rustc in the future that has"] # [doc = " additional levels represented by an integer that are not known by this"] # [doc = " version of `cargo_metadata`."] UnknownInt (i64) , # [doc = " An unknown string level."] # [doc = ""] # [doc = " This may be produced by a version of rustc in the future that has"] # [doc = " additional levels represented by a string that are not known by this"] # [doc = " version of `cargo_metadata`."] UnknownString (String) , }
    };
}

ArtifactDebuginfo!()