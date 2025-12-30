// Generated macro for tests (module)
macro_rules! Depcrate_registry_export_sdltests {
() => {
// Module: crate::registry::export_sdl
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: { model :: __DirectiveLocation , registry :: MetaDirective } ; # [test] fn test_escape_string () { assert_eq ! (escape_string ("1\\\x08d\x0c3\n4\r5\t6") , "1\\\\\\bd\\f3\\n4\\r5\\t6") ; } # [test] fn test_compose_directive_dsl () { let expected = r#"directive @custom_type_directive on FIELD_DEFINITION
extend schema @link(
	url: "https://specs.apollo.dev/federation/v2.5",
	import: ["@key", "@tag", "@shareable", "@inaccessible", "@override", "@external", "@provides", "@requires", "@composeDirective", "@interfaceObject", "@requiresScopes"]
)

extend schema @link(
	url: "https://custom.spec.dev/extension/v1.0"
	import: ["@custom_type_directive"]
)
	@composeDirective(name: "@custom_type_directive")

"# ; let mut registry = Registry :: default () ; registry . add_directive (MetaDirective { name : "custom_type_directive" . to_string () , description : None , locations : vec ! [__DirectiveLocation :: FIELD_DEFINITION] , args : Default :: default () , is_repeatable : false , visible : None , composable : Some ("https://custom.spec.dev/extension/v1.0" . to_string ()) , }) ; let dsl = registry . export_sdl (SDLExportOptions :: new () . federation () . compose_directive ()) ; assert_eq ! (dsl , expected) } # [test] fn test_type_directive_sdl_without_federation () { let expected = r#"directive @custom_type_directive(optionalWithoutDefault: String, optionalWithDefault: String = "DEFAULT") on FIELD_DEFINITION | OBJECT
schema {
	query: Query
}
"# ; let mut registry = Registry :: default () ; registry . add_directive (MetaDirective { name : "custom_type_directive" . to_string () , description : None , locations : vec ! [__DirectiveLocation :: FIELD_DEFINITION , __DirectiveLocation :: OBJECT ,] , args : [("optionalWithoutDefault" . to_string () , MetaInputValue { name : "optionalWithoutDefault" . to_string () , description : None , ty : "String" . to_string () , deprecation : Deprecation :: NoDeprecated , default_value : None , visible : None , inaccessible : false , tags : vec ! [] , is_secret : false , directive_invocations : vec ! [] , } ,) , ("optionalWithDefault" . to_string () , MetaInputValue { name : "optionalWithDefault" . to_string () , description : None , ty : "String" . to_string () , deprecation : Deprecation :: NoDeprecated , default_value : Some ("\"DEFAULT\"" . to_string ()) , visible : None , inaccessible : false , tags : vec ! [] , is_secret : false , directive_invocations : vec ! [] , } ,) ,] . into () , is_repeatable : false , visible : None , composable : None , }) ; registry . query_type = "Query" . to_string () ; let sdl = registry . export_sdl (SDLExportOptions :: new ()) ; assert_eq ! (sdl , expected) } }
};
}
