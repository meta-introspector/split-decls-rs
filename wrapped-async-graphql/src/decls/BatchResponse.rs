macro_rules! deps {
    () => {
        Response!();
    };
}

macro_rules! BatchResponse {
    () => {
        deps!();
        # [doc = " Response for batchable queries"] # [allow (clippy :: large_enum_variant)] # [derive (Debug , Serialize)] # [serde (untagged)] pub enum BatchResponse { # [doc = " Response for single queries"] Single (Response) , # [doc = " Response for batch queries"] Batch (Vec < Response >) , }
    };
}

BatchResponse!()