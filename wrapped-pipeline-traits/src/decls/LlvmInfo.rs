macro_rules! LlvmInfo {
    () => {
        # [derive (Debug , Clone)] pub struct LlvmInfo { pub ir_version : String , pub target_triple : String , }
    };
}

LlvmInfo!()