macro_rules! deps {
    () => {
        Data!();
        Registry!();
        CustomDirectiveFactory!();
    };
}

macro_rules! SchemaEnvInner {
    () => {
        deps!();
        # [doc (hidden)] pub struct SchemaEnvInner { pub registry : Registry , pub data : Data , pub custom_directives : HashMap < String , Box < dyn CustomDirectiveFactory > > , }
    };
}

SchemaEnvInner!()