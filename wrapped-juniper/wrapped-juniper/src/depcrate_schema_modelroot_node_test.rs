// Generated macro for root_node_test (module)
macro_rules! Depcrate_schema_modelroot_node_test {
() => {
// Module: crate::schema::model
// Provides: {"root_node_test"}
// Dependencies: {}
# [cfg (test)] mod root_node_test { # [cfg (feature = "schema-language")] mod as_document { use crate :: { EmptyMutation , EmptySubscription , RootNode , graphql_object } ; struct Query ; # [graphql_object] impl Query { fn blah () -> bool { true } } # [test] fn generates_correct_document () { let schema = RootNode :: new (Query , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let ast = graphql_parser :: parse_schema :: < & str > (r#"
                type Query {
                    blah: Boolean!
                }

                schema {
                  query: Query
                }
                "# ,) . unwrap () ; assert_eq ! (ast . to_string () , schema . as_document () . to_string ()) ; } } # [cfg (feature = "schema-language")] mod as_sdl { use crate :: { EmptyMutation , EmptySubscription , GraphQLEnum , GraphQLInputObject , GraphQLObject , GraphQLUnion , RootNode , graphql_object , } ; # [derive (GraphQLObject , Default)] struct Cake { fresh : bool , } # [derive (GraphQLObject , Default)] struct IceCream { cold : bool , } # [derive (GraphQLUnion)] enum GlutenFree { Cake (Cake) , IceCream (IceCream) , } # [derive (GraphQLEnum)] enum Fruit { Apple , Orange , } # [derive (GraphQLInputObject)] struct Coordinate { latitude : f64 , longitude : f64 , } struct Query ; # [graphql_object] impl Query { fn blah () -> bool { true } # [doc = " This is whatever's description."] fn whatever () -> String { "foo" . into () } fn arr (stuff : Vec < Coordinate >) -> Option < & 'static str > { (! stuff . is_empty ()) . then_some ("stuff") } fn fruit () -> Fruit { Fruit :: Apple } fn gluten_free (flavor : String) -> GlutenFree { if flavor == "savory" { GlutenFree :: Cake (Cake :: default ()) } else { GlutenFree :: IceCream (IceCream :: default ()) } } # [deprecated] fn old () -> i32 { 42 } # [deprecated (note = "This field is deprecated, use another.")] fn really_old () -> f64 { 42.0 } } # [test] fn generates_correct_sdl () { let actual = RootNode :: new (Query , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let expected = graphql_parser :: parse_schema :: < & str > (r#"
                schema {
                  query: Query
                }
                enum Fruit {
                    APPLE
                    ORANGE
                }
                input Coordinate {
                    latitude: Float!
                    longitude: Float!
                }
                type Cake {
                    fresh: Boolean!
                }
                type IceCream {
                    cold: Boolean!
                }
                type Query {
                  blah: Boolean!
                  "This is whatever's description."
                  whatever: String!
                  arr(stuff: [Coordinate!]!): String
                  fruit: Fruit!
                  glutenFree(flavor: String!): GlutenFree!
                  old: Int! @deprecated
                  reallyOld: Float! @deprecated(reason: "This field is deprecated, use another.")
                }
                union GlutenFree = Cake | IceCream
                "# ,) . unwrap () ; assert_eq ! (actual . as_sdl () , expected . to_string ()) ; } } }
};
}
