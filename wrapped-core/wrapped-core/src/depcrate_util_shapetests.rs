// Generated macro for tests (module)
macro_rules! Depcrate_util_shapetests {
() => {
// Module: crate::util::shape
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use syn :: parse_quote ; use super :: * ; # [test] fn any_accepts_anything () { let mut filter = ShapeSet :: default () ; filter . insert_all () ; let unit_struct : syn :: DeriveInput = syn :: parse_quote ! { struct Example ; } ; if let syn :: Data :: Struct (data) = unit_struct . data { assert ! (filter . contains (& data)) ; } else { panic ! ("Struct not parsed as struct") ; } ; } # [test] fn tuple_accepts_newtype () { let filter = ShapeSet :: new (vec ! [Shape :: Tuple]) ; let newtype_struct : syn :: DeriveInput = parse_quote ! { struct Example (String) ; } ; if let syn :: Data :: Struct (data) = newtype_struct . data { assert ! (filter . contains (& data)) ; } else { panic ! ("Struct not parsed as struct") ; } ; } # [test] fn newtype_rejects_tuple () { let filter = ShapeSet :: new (vec ! [Shape :: Newtype]) ; let tuple_struct : syn :: DeriveInput = parse_quote ! { struct Example (String , u64) ; } ; if let syn :: Data :: Struct (data) = tuple_struct . data { assert ! (! filter . contains (& data)) ; } else { panic ! ("Struct not parsed as struct") ; } ; } }
};
}
