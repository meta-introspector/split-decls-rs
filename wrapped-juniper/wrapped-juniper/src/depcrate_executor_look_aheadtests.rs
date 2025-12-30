// Generated macro for tests (module)
macro_rules! Depcrate_executor_look_aheadtests {
() => {
// Module: crate::executor::look_ahead
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: collections :: HashMap ; use crate :: { ast :: { Document , OwnedDocument } , graphql , parser :: UnlocatedParseResult , schema :: model :: SchemaType , validation :: test_harness :: { MutationRoot , QueryRoot , SubscriptionRoot } , value :: { DefaultScalarValue , ScalarValue } , } ; use super :: * ; fn parse_document_source < S > (q : & str) -> UnlocatedParseResult < OwnedDocument < '_ , S > > where S : ScalarValue , { crate :: parse_document_source (q , & SchemaType :: new :: < QueryRoot , MutationRoot , SubscriptionRoot > (& () , & () , & ()) ,) } fn extract_fragments < 'a , S > (doc : & 'a Document < S >) -> HashMap < & 'a str , Fragment < 'a , S > > where S : Clone , { let mut fragments = HashMap :: new () ; for d in doc { if let crate :: ast :: Definition :: Fragment (ref f) = * d { let f = f . item . clone () ; fragments . insert (f . name . item , f) ; } } fragments } fn selection_look_ahead < 'a , S : ScalarValue > (selection : & 'a Selection < 'a , S > , vars : & 'a Variables < S > , fragments : & 'a HashMap < & 'a str , Fragment < 'a , S > > ,) -> LookAheadSelection < 'a , S > { let mut collector = ChildrenBuilder { vars , fragments , type_filter : Applies :: All , output : vec ! [] , } ; collector . visit_child (selection , Applies :: All) ; collector . output . into_iter () . next () . unwrap () } # [derive (Debug , PartialEq)] enum ValueDebug < 'a , S : ScalarValue > { Null , Scalar (& 'a S) , Enum (& 'a str) , List (Vec < ValueDebug < 'a , S > >) , Object (Vec < (& 'a str , ValueDebug < 'a , S >) >) , } impl < 'a , S : ScalarValue > From < LookAheadValue < 'a , S > > for ValueDebug < 'a , S > { fn from (look_ahead : LookAheadValue < 'a , S >) -> Self { match look_ahead { LookAheadValue :: Null => Self :: Null , LookAheadValue :: Scalar (s) => Self :: Scalar (s) , LookAheadValue :: Enum (e) => Self :: Enum (e) , LookAheadValue :: List (list) => { Self :: List (list . iter () . map (| val | val . item . into ()) . collect ()) } LookAheadValue :: Object (object) => Self :: Object (object . iter () . map (| (key , value) | (key . item , value . item . into ())) . collect () ,) , } } } # [derive (Debug , PartialEq)] struct LookAheadDebug < 'a , S : ScalarValue > { name : & 'a str , alias : Option < & 'a str > , applies_for : Applies < 'a > , arguments : Option < Vec < (& 'a str , ValueDebug < 'a , S >) > > , children : Vec < LookAheadDebug < 'a , S > > , } impl < 'a , S : ScalarValue > LookAheadDebug < 'a , S > { fn new (look_ahead : & LookAheadSelection < 'a , S >) -> Self { Self :: new_filtered (look_ahead , Applies :: All) } fn new_filtered (look_ahead : & LookAheadSelection < 'a , S > , type_filter : Applies) -> Self { Self { name : look_ahead . field_name () , alias : look_ahead . field_alias () , applies_for : look_ahead . applies_for , arguments : if look_ahead . has_arguments () { Some (look_ahead . arguments () . map (| argument | (argument . name () , ValueDebug :: from (argument . value ()))) . collect () ,) } else { None } , children : look_ahead . build_children (type_filter) . iter () . map (| child | Self :: new_filtered (child , type_filter)) . collect () , } } } # [test] fn check_simple_query () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero {
                hero {
                    id
                    name
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , applies_for : Applies :: All , arguments : None , children : vec ! [LookAheadDebug { name : "id" , alias : None , arguments : None , children : vec ! [] , applies_for : Applies :: All , } , LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } ,] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_query_with_child () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero {
                hero {
                    id
                    name
                    friends {
                        name
                        id
                    }
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "friends" , alias : None , arguments : None , children : vec ! [LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } ,] , applies_for : Applies :: All , } ,] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_query_with_argument () { let docs = parse_document_source ("
            query Hero {
                hero(episode: EMPIRE) {
                    id
                    name(uppercase: true)
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : Some (vec ! [("episode" , ValueDebug :: Enum ("EMPIRE"))]) , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "name" , alias : None , arguments : Some (vec ! [("uppercase" , ValueDebug :: Scalar (& DefaultScalarValue :: Boolean (true)) ,)]) , children : Vec :: new () , applies_for : Applies :: All , } ,] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_query_with_variable () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero($episode: Episode) {
                hero(episode: $episode) {
                    id
                    name
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { "episode" : JEDI } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : Some (vec ! [("episode" , ValueDebug :: Enum ("JEDI"))]) , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } ,] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_query_with_optional_variable () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero($episode: Episode) {
                hero(episode: $episode) {
                    id
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : Some (vec ! [("episode" , ValueDebug :: Null)]) , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , }] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_query_with_fragment () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero {
                hero {
                    id
                    ...commonFields
                }
            }

            fragment commonFields on Character {
                name
                appearsIn
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "appearsIn" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } ,] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_query_with_directives () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero {
                hero {
                    id @include(if: true)
                    name @include(if: false)
                    appearsIn @skip(if: true)
                    height @skip(if: false)
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "height" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } ,] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_query_with_inline_fragments () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero {
                hero {
                    name
                    ... on Droid {
                        primaryFunction
                    }
                    ... on Human {
                        height
                    }
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "primaryFunction" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: OnlyType ("Droid") , } , LookAheadDebug { name : "height" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: OnlyType ("Human") , } ,] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_query_with_multiple () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query HeroAndHuman {
                hero {
                    id
                }
                human {
                    name
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , }] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; let look_ahead = selection_look_ahead (& op . item . selection_set [1] , & vars , & fragments) ; let expected = LookAheadDebug { name : "human" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , }] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_complex_query () { let docs = parse_document_source ("
            query HeroNameAndFriends($id: Integer!, $withFriends: Boolean! = true) {
                hero(id: $id) {
                    id
                    ... comparisonFields
                    friends @include(if: $withFriends) {
                        ... comparisonFields
                        ... on Human @skip(if: true) { mass }
                    }
                }
            }

            fragment comparisonFields on Character {
                __typename
                name
                appearsIn
                ... on Droid { primaryFunction }
                ... on Human { height }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { "id" : 42 , "withFriends" : true , } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : Some (vec ! [("id" , ValueDebug :: Scalar (& DefaultScalarValue :: Int (42)) ,)]) , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "__typename" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "appearsIn" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "primaryFunction" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: OnlyType ("Droid") , } , LookAheadDebug { name : "height" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: OnlyType ("Human") , } , LookAheadDebug { name : "friends" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "__typename" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "appearsIn" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "primaryFunction" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: OnlyType ("Droid") , } , LookAheadDebug { name : "height" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: OnlyType ("Human") , } ,] , } ,] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_resolve_concrete_type () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero {
                hero {
                    name
                    ... on Droid {
                        primaryFunction
                    }
                    ... on Human {
                        height
                    }
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "height" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: OnlyType ("Human") , } ,] , } ; assert_eq ! (LookAheadDebug :: new_filtered (& look_ahead , Applies :: OnlyType ("Human")) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_select_child () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero {
                hero {
                    id
                    friends {
                        id
                        name
                    }
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let id = look_ahead . children () . select ("id") . unwrap () ; let concrete_id = look_ahead . children_for_explicit_type ("does not matter") . select ("id") . unwrap () ; let expected = LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } ; assert_eq ! (LookAheadDebug :: new (& id) , expected) ; assert_eq ! (LookAheadDebug :: new (& concrete_id) , expected) ; let friends = look_ahead . children () . select ("friends") . unwrap () ; let concrete_friends = look_ahead . children_for_explicit_type ("does not matter") . select ("friends") . unwrap () ; let expected = LookAheadDebug { name : "friends" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "id" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } , LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , } ,] , } ; assert_eq ! (LookAheadDebug :: new (& friends) , expected) ; assert_eq ! (LookAheadDebug :: new (& concrete_friends) , expected) ; } } # [test] fn check_fragment_with_nesting () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero {
                hero {
                    ...heroFriendNames
                }
            }

            fragment heroFriendNames on Hero {
                friends { name }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let expected = LookAheadDebug { name : "hero" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "friends" , alias : None , arguments : None , applies_for : Applies :: All , children : vec ! [LookAheadDebug { name : "name" , alias : None , arguments : None , children : Vec :: new () , applies_for : Applies :: All , }] , }] , } ; assert_eq ! (LookAheadDebug :: new (& look_ahead) , expected) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_visitability () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero {
                hero(episode: EMPIRE) {
                    name
                    aliasedName: name
                    friends {
                        name
                    }
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; assert_eq ! (look_ahead . field_original_name () , "hero") ; assert ! (look_ahead . field_alias () . is_none ()) ; assert_eq ! (look_ahead . field_name () , "hero") ; assert ! (look_ahead . has_arguments ()) ; let arg = look_ahead . arguments () . next () . unwrap () ; assert_eq ! (arg . name () , "episode") ; assert_eq ! (ValueDebug :: from (arg . value ()) , ValueDebug :: Enum ("EMPIRE")) ; let children = look_ahead . children () ; assert ! (! children . is_empty ()) ; assert_eq ! (children . names () . collect ::< Vec < _ >> () , vec ! ["name" , "aliasedName" , "friends"] ,) ; let mut child_iter = children . iter () ; let name_child = child_iter . next () . unwrap () ; assert ! (children . has_child ("name")) ; assert_eq ! (LookAheadDebug :: new (name_child) , LookAheadDebug :: new (& children . select ("name") . unwrap ())) ; assert_eq ! (name_child . field_original_name () , "name") ; assert_eq ! (name_child . field_alias () , None) ; assert_eq ! (name_child . field_name () , "name") ; assert ! (! name_child . has_arguments ()) ; assert ! (name_child . children () . is_empty ()) ; let aliased_name_child = child_iter . next () . unwrap () ; assert ! (children . has_child ("aliasedName")) ; assert_eq ! (LookAheadDebug :: new (aliased_name_child) , LookAheadDebug :: new (& children . select ("aliasedName") . unwrap ())) ; assert_eq ! (aliased_name_child . field_original_name () , "name") ; assert_eq ! (aliased_name_child . field_alias () , Some ("aliasedName")) ; assert_eq ! (aliased_name_child . field_name () , "aliasedName") ; assert ! (! aliased_name_child . has_arguments ()) ; assert ! (aliased_name_child . children () . is_empty ()) ; let friends_child = child_iter . next () . unwrap () ; assert ! (children . has_child ("friends")) ; assert_eq ! (LookAheadDebug :: new (friends_child) , LookAheadDebug :: new (& children . select ("friends") . unwrap ())) ; assert_eq ! (friends_child . field_original_name () , "friends") ; assert_eq ! (friends_child . field_alias () , None) ; assert_eq ! (friends_child . field_name () , "friends") ; assert ! (! friends_child . has_arguments ()) ; assert ! (! friends_child . children () . is_empty ()) ; assert_eq ! (friends_child . children () . names () . collect ::< Vec < _ >> () , vec ! ["name"] ,) ; assert ! (child_iter . next () . is_none ()) ; let friends_children = friends_child . children () ; let mut friends_child_iter = friends_children . iter () ; let child = friends_child_iter . next () . unwrap () ; assert ! (friends_children . has_child ("name")) ; assert_eq ! (LookAheadDebug :: new (child) , LookAheadDebug :: new (& children . select ("name") . unwrap ())) ; assert_eq ! (child . field_original_name () , "name") ; assert_eq ! (child . field_alias () , None) ; assert_eq ! (child . field_name () , "name") ; assert ! (! child . has_arguments ()) ; assert ! (child . children () . is_empty ()) ; assert ! (friends_child_iter . next () . is_none ()) ; } else { panic ! ("No Operation found") ; } } # [test] fn check_resolves_applies_for () { let docs = parse_document_source :: < DefaultScalarValue > ("
            query Hero {
                hero {
                    ... on Human {
                        height
                    }
                }
            }
            " ,) . unwrap () ; let fragments = extract_fragments (& docs) ; if let crate :: ast :: Definition :: Operation (ref op) = docs [0] { let vars = graphql :: vars ! { } ; let look_ahead = selection_look_ahead (& op . item . selection_set [0] , & vars , & fragments) ; let mut children = look_ahead . children_for_explicit_type ("Human") . into_iter () ; let heights_child = children . next () . unwrap () ; assert_eq ! (heights_child . field_original_name () , "height") ; assert_eq ! (heights_child . applies_for , Applies :: OnlyType ("Human")) ; assert_eq ! (heights_child . applies_for () . unwrap () , "Human") ; } else { panic ! ("No Operation found") ; } } }
};
}
