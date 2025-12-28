macro_rules! deps {
    () => {
        Extensions!();
    };
}

macro_rules! ErrorExtensionValues {
    () => {
        deps!();
        # [doc = " Extensions to the error."] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize , Default)] # [serde (transparent)] pub struct ErrorExtensionValues (BTreeMap < String , Value >) ;
    };
}

ErrorExtensionValues!()