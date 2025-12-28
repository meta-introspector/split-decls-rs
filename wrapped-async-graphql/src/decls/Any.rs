macro_rules! Any {
    () => {
        # [doc = " Any scalar (For [Apollo Federation](https://www.apollographql.com/docs/apollo-server/federation/introduction))"] # [doc = ""] # [doc = " The `Any` scalar is used to pass representations of entities from external"] # [doc = " services into the root `_entities` field for execution."] # [derive (Clone , Eq , PartialEq , Debug)] pub struct Any (pub Value) ;
    };
}

Any!();