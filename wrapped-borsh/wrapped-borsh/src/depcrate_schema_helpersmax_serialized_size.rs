// Generated macro for max_serialized_size (function)
macro_rules! Depcrate_schema_helpersmax_serialized_size {
() => {
// Module: crate::schema_helpers
// Provides: {"max_serialized_size"}
// Dependencies: {}
# [doc = " Returns the largest possible size of a serialised object based solely on its type `T`."] # [doc = ""] # [doc = " this is a shortcut for using [BorshSchemaContainer::max_serialized_size]"] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use borsh::schema::BorshSchemaContainer;"] # [doc = ""] # [doc = " assert_eq!(Ok(8), borsh::max_serialized_size::<usize>());"] # [doc = " ```"] pub fn max_serialized_size < T : BorshSchema + ? Sized > () -> core :: result :: Result < usize , SchemaMaxSerializedSizeError > { let schema = BorshSchemaContainer :: for_type :: < T > () ; schema . max_serialized_size () }
};
}
