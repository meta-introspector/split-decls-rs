macro_rules! deps {
    () => {
        SchemaInner!();
    };
}

macro_rules! Schema {
    () => {
        deps!();
        # [doc = " Dynamic GraphQL schema."] # [doc = ""] # [doc = " Cloning a schema is cheap, so it can be easily shared."] # [derive (Clone)] pub struct Schema (pub (crate) Arc < SchemaInner >) ;
    };
}

Schema!()