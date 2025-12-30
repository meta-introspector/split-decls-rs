// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_extensionsvalidate {
() => {
// Module: crate::config::tree::sections::extensions
// Provides: {"validate"}
// Dependencies: {}
mod validate { use crate :: { bstr :: BStr , config :: tree :: keys } ; pub struct ObjectFormat ; impl keys :: Validate for ObjectFormat { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { super :: Extensions :: OBJECT_FORMAT . try_into_object_format (value . into ()) ? ; Ok (()) } } }
};
}
