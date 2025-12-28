macro_rules! deps {
    () => {
        FieldValue!();
        Request!();
    };
}

macro_rules! DynamicRequest {
    () => {
        deps!();
        # [doc = " GraphQL request for dynamic schema."] pub struct DynamicRequest { pub (crate) inner : Request , pub (crate) root_value : FieldValue < 'static > , }
    };
}

DynamicRequest!()