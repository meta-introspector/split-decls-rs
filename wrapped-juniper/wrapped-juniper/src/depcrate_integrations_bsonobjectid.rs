// Generated macro for ObjectId (type)
macro_rules! Depcrate_integrations_bsonObjectId {
() => {
// Module: crate::integrations::bson
// Provides: {"ObjectId"}
// Dependencies: {}
# [doc = " [BSON ObjectId][0] represented as a HEX string."] # [doc = ""] # [doc = " [`ObjectID` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`bson::oid::ObjectId`][2] for details."] # [doc = ""] # [doc = " [0]: https://www.mongodb.com/docs/manual/reference/bson-types#objectid"] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/object-id"] # [doc = " [2]: https://docs.rs/bson/*/bson/oid/struct.ObjectId.html"] # [graphql_scalar] # [graphql (name = "ObjectID" , with = object_id , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/object-id" ,)] type ObjectId = bson :: oid :: ObjectId ;
};
}
