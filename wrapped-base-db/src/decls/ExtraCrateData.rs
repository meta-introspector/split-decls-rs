macro_rules! deps {
    () => {
        Dependency!();
        CrateDisplayName!();
        Crate!();
    };
}

macro_rules! ExtraCrateData {
    () => {
        deps!();
        # [doc = " Crate data unrelated to analysis."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct ExtraCrateData { pub version : Option < String > , # [doc = " A name used in the package's project declaration: for Cargo projects,"] # [doc = " its `[package].name` can be different for other project types or even"] # [doc = " absent (a dummy crate for the code snippet, for example)."] # [doc = ""] # [doc = " For purposes of analysis, crates are anonymous (only names in"] # [doc = " `Dependency` matters), this name should only be used for UI."] pub display_name : Option < CrateDisplayName > , # [doc = " The cfg options that could be used by the crate"] pub potential_cfg_options : Option < CfgOptions > , }
    };
}

ExtraCrateData!();