macro_rules! deps {
    () => {
        CrateWorkspaceData!();
        Env!();
        CrateDataBuilder!();
        ExtraCrateData!();
    };
}

macro_rules! CrateBuilder {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct CrateBuilder { pub basic : CrateDataBuilder , pub extra : ExtraCrateData , pub cfg_options : CfgOptions , pub env : Env , ws_data : Arc < CrateWorkspaceData > , }
    };
}

CrateBuilder!()