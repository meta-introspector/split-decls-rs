macro_rules! deps {
    () => {
        Name!();
        ResourceName!();
        Id!();
    };
}

macro_rules! ResourceNameOrId {
    () => {
        deps!();
        # [doc = " A resource name or ID."] # [doc = ""] # [doc = " Can be either a string or a numeric ID."] # [derive (Debug)] pub enum ResourceNameOrId { # [doc = " A resource name."] Name (ResourceName) , # [doc = " A resource ID."] Id (u16) , }
    };
}

ResourceNameOrId!()