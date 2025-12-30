// Generated macro for properties (function)
macro_rules! Depcrate_stdimplproperties {
() => {
// Module: crate::stdimpl
// Provides: {"properties"}
// Dependencies: {}
pub fn properties (cr : & mut Crossroads) -> IfaceToken < () > { cr . register ("org.freedesktop.DBus.Properties" , | b | { b . method_with_cr_custom :: < _ , (Variant < u8 > ,) , _ , _ > ("Get" , ("interface_name" , "property_name") , ("value" ,) , get) ; b . method_with_cr_custom :: < _ , (PropMap ,) , _ , _ > ("GetAll" , ("interface_name" ,) , ("properties" ,) , getall) ; b . method_with_cr_custom :: < _ , () , _ , _ > ("Set" , ("interface_name" , "property_name" , "value") , () , set) ; b . signal :: < (String , PropMap , Vec < String >) , _ > ("PropertiesChanged" , ("interface_name" , "changed_properties" , "invalidated_properties")) ; }) }
};
}
