macro_rules! deps {
    () => {
        Env!();
        ExtraCrateData!();
        BuiltCrateData!();
        CrateWorkspaceData!();
    };
}

macro_rules! Crate {
    () => {
        deps!();
        # [salsa_macros :: input] # [derive (Debug , PartialOrd , Ord)] pub struct Crate { # [returns (ref)] pub data : BuiltCrateData , # [doc = " Crate data that is not needed for analysis."] # [doc = ""] # [doc = " This is split into a separate field to increase incrementality."] # [returns (ref)] pub extra_data : ExtraCrateData , # [returns (ref)] pub workspace_data : Arc < CrateWorkspaceData > , # [returns (ref)] pub cfg_options : CfgOptions , # [returns (ref)] pub env : Env , }
    };
}

Crate!()