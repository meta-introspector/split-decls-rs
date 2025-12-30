// Generated macro for private (module)
macro_rules! Depcrate_connectionprivate {
() => {
// Module: crate::connection
// Provides: {"private"}
// Dependencies: {}
pub (crate) mod private { # [doc = " This trait restricts who can implement `Connection`"] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] pub trait ConnectionSealed { } # [doc = " This trait provides helper methods to convert a database lookup type"] # [doc = " to/from an `std::any::Any` reference. This is used internally by the `#[derive(MultiConnection)]`"] # [doc = " implementation"] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] pub trait MultiConnectionHelper : super :: Connection { # [doc = " Convert the lookup type to any"] fn to_any < 'a > (lookup : & mut < Self :: Backend as crate :: sql_types :: TypeMetadata > :: MetadataLookup ,) -> & mut (dyn std :: any :: Any + 'a) ; # [doc = " Get the lookup type from any"] fn from_any (lookup : & mut dyn std :: any :: Any ,) -> Option < & mut < Self :: Backend as crate :: sql_types :: TypeMetadata > :: MetadataLookup > ; } # [allow (unreachable_pub)] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] pub trait ConnectionHelperType < DB , B > : super :: LoadConnection < B , Backend = DB > { type Cursor < 'conn , 'query > where Self : 'conn ; } # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] impl < T , B > ConnectionHelperType < T :: Backend , B > for T where T : super :: LoadConnection < B > , { type Cursor < 'conn , 'query > = < T as super :: LoadConnection < B > > :: Cursor < 'conn , 'query > where T : 'conn ; } }
};
}
