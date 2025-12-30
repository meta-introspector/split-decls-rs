// Generated macro for discriminant_value (function)
macro_rules! Depcratediscriminant_value {
() => {
// Module: crate
// Provides: {"discriminant_value"}
// Dependencies: {}
# [doc = " Gets the serialization discriminant if specified."] fn discriminant_value (attrs : & [Attribute]) -> Result < Option < DiscriminantValue > > { TlsAttr :: parse_multi (attrs) ? . into_iter () . try_fold (None , | discriminant , attr | match (discriminant , attr) { (None , TlsAttr :: Discriminant (d)) => Ok (Some (d)) , (Some (_) , TlsAttr :: Discriminant (_)) => Err (syn :: Error :: new (Span :: call_site () , "Attribute `discriminant` specified more than once" ,)) , (_ , attr) => Err (syn :: Error :: new (Span :: call_site () , format ! ("Unrecognized variant attribute `{}`" , attr . name ()) ,)) , }) }
};
}
