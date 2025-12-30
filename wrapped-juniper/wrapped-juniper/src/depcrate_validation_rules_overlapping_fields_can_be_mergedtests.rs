// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_overlapping_fields_can_be_mergedtests {
() => {
// Module: crate::validation::rules::overlapping_fields_can_be_merged
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use arcstr :: ArcStr ; use super :: { ConflictReason , ConflictReasonMessage :: * , error_message , factory } ; use crate :: { executor :: Registry , schema :: meta :: MetaType , types :: { base :: { GraphQLType , GraphQLValue } , scalars :: { EmptyMutation , EmptySubscription , ID } , } , } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_fails_rule_with_schema , expect_passes_rule , expect_passes_rule_with_schema , } , value :: { DefaultScalarValue , ScalarValue } , } ; # [test] fn unique_fields () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment uniqueFields on Dog {
            name
            nickname
          }
        "# ,) ; } # [test] fn identical_fields () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment mergeIdenticalFields on Dog {
            name
            name
          }
        "# ,) ; } # [test] fn identical_fields_with_identical_args () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment mergeIdenticalFieldsWithIdenticalArgs on Dog {
            doesKnowCommand(dogCommand: SIT)
            doesKnowCommand(dogCommand: SIT)
          }
        "# ,) ; } # [test] fn identical_fields_with_identical_directives () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment mergeSameFieldsWithSameDirectives on Dog {
            name @include(if: true)
            name @include(if: true)
          }
        "# ,) ; } # [test] fn different_args_with_different_aliases () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment differentArgsWithDifferentAliases on Dog {
            knowsSit: doesKnowCommand(dogCommand: SIT)
            knowsDown: doesKnowCommand(dogCommand: DOWN)
          }
        "# ,) ; } # [test] fn different_directives_with_different_aliases () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment differentDirectivesWithDifferentAliases on Dog {
            nameIfTrue: name @include(if: true)
            nameIfFalse: name @include(if: false)
          }
        "# ,) ; } # [test] fn different_skip_include_directives_accepted () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment differentDirectivesWithDifferentAliases on Dog {
            name @include(if: true)
            name @include(if: false)
          }
        "# ,) ; } # [test] fn same_aliases_with_different_field_targets () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment sameAliasesWithDifferentFieldTargets on Dog {
            fido: name
            fido: nickname
          }
        "# , & [RuleError :: new (& error_message ("fido" , & Message ("name and nickname are different fields" . into ()) ,) , & [SourcePosition :: new (78 , 2 , 12) , SourcePosition :: new (101 , 3 , 12) ,] ,)] ,) ; } # [test] fn same_aliases_allowed_on_nonoverlapping_fields () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment sameAliasesWithDifferentFieldTargets on Pet {
            ... on Dog {
              name
            }
            ... on Cat {
              name: nickname
            }
          }
        "# ,) ; } # [test] fn alias_masking_direct_field_access () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment aliasMaskingDirectFieldAccess on Dog {
            name: nickname
            name
          }
        "# , & [RuleError :: new (& error_message ("name" , & Message ("nickname and name are different fields" . into ()) ,) , & [SourcePosition :: new (71 , 2 , 12) , SourcePosition :: new (98 , 3 , 12) ,] ,)] ,) ; } # [test] fn different_args_second_adds_an_argument () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment conflictingArgs on Dog {
            doesKnowCommand
            doesKnowCommand(dogCommand: HEEL)
          }
        "# , & [RuleError :: new (& error_message ("doesKnowCommand" , & Message ("they have differing arguments" . into ()) ,) , & [SourcePosition :: new (57 , 2 , 12) , SourcePosition :: new (85 , 3 , 12) ,] ,)] ,) ; } # [test] fn different_args_second_missing_an_argument () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment conflictingArgs on Dog {
            doesKnowCommand(dogCommand: SIT)
            doesKnowCommand
          }
        "# , & [RuleError :: new (& error_message ("doesKnowCommand" , & Message ("they have differing arguments" . into ()) ,) , & [SourcePosition :: new (57 , 2 , 12) , SourcePosition :: new (102 , 3 , 12) ,] ,)] ,) ; } # [test] fn conflicting_args () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment conflictingArgs on Dog {
            doesKnowCommand(dogCommand: SIT)
            doesKnowCommand(dogCommand: HEEL)
          }
        "# , & [RuleError :: new (& error_message ("doesKnowCommand" , & Message ("they have differing arguments" . into ()) ,) , & [SourcePosition :: new (57 , 2 , 12) , SourcePosition :: new (102 , 3 , 12) ,] ,)] ,) ; } # [test] fn allows_different_args_where_no_conflict_is_possible () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment conflictingArgs on Pet {
            ... on Dog {
              name(surname: true)
            }
            ... on Cat {
              name
            }
          }
        "# ,) ; } # [test] fn encounters_conflict_in_fragments () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            ...A
            ...B
          }
          fragment A on Dog {
            x: name
          }
          fragment B on Dog {
            x: barks
          }
        "# , & [RuleError :: new (& error_message ("x" , & Message ("name and barks are different fields" . into ())) , & [SourcePosition :: new (101 , 6 , 12) , SourcePosition :: new (163 , 9 , 12) ,] ,)] ,) ; } # [test] fn reports_each_conflict_once () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dorOrHuman {
              ...A
              ...B
            }
            catOrDog {
              ...B
              ...A
            }
            dog {
              ...A
              ...B
              x: name
            }
          }
          fragment A on Dog {
            x: barks
          }
          fragment B on Dog {
            x: nickname
          }
        "# , & [RuleError :: new (& error_message ("x" , & Message ("name and barks are different fields" . into ())) , & [SourcePosition :: new (235 , 13 , 14) , SourcePosition :: new (311 , 17 , 12) ,] ,) , RuleError :: new (& error_message ("x" , & Message ("name and nickname are different fields" . into ()) ,) , & [SourcePosition :: new (235 , 13 , 14) , SourcePosition :: new (374 , 20 , 12) ,] ,) , RuleError :: new (& error_message ("x" , & Message ("barks and nickname are different fields" . into ()) ,) , & [SourcePosition :: new (311 , 17 , 12) , SourcePosition :: new (374 , 20 , 12) ,] ,) ,] ,) ; } # [test] fn deep_conflict () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog {
              x: name
            },
            dog {
              x: barks
            }
          }
        "# , & [RuleError :: new (& error_message ("dog" , & Nested (vec ! [ConflictReason ("x" . into () , Message ("name and barks are different fields" . into ()) ,)]) ,) , & [SourcePosition :: new (25 , 2 , 12) , SourcePosition :: new (45 , 3 , 14) , SourcePosition :: new (80 , 5 , 12) , SourcePosition :: new (100 , 6 , 14) ,] ,)] ,) ; } # [test] fn deep_conflict_with_multiple_issues () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog {
              x: barks
              y: name
            },
              dog {
              x: nickname
              y: barkVolume
            }
          }
        "# , & [RuleError :: new (& error_message ("dog" , & Nested (vec ! [ConflictReason ("x" . into () , Message ("barks and nickname are different fields" . into ()) ,) , ConflictReason ("y" . into () , Message ("name and barkVolume are different fields" . into ()) ,) ,]) ,) , & [SourcePosition :: new (25 , 2 , 12) , SourcePosition :: new (45 , 3 , 14) , SourcePosition :: new (68 , 4 , 14) , SourcePosition :: new (105 , 6 , 14) , SourcePosition :: new (125 , 7 , 14) , SourcePosition :: new (151 , 8 , 14) ,] ,)] ,) ; } # [test] fn very_deep_conflict () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            human {
              relatives {
                x: name
              }
            },
            human {
              relatives {
                x: iq
              }
            }
          }
        "# , & [RuleError :: new (& error_message ("human" , & Nested (vec ! [ConflictReason ("relatives" . into () , Nested (vec ! [ConflictReason ("x" . into () , Message ("name and iq are different fields" . into ()) ,)]) ,)]) ,) , & [SourcePosition :: new (25 , 2 , 12) , SourcePosition :: new (47 , 3 , 14) , SourcePosition :: new (75 , 4 , 16) , SourcePosition :: new (126 , 7 , 12) , SourcePosition :: new (148 , 8 , 14) , SourcePosition :: new (176 , 9 , 16) ,] ,)] ,) ; } # [test] fn reports_deep_conflict_to_nearest_common_ancestor () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            human {
              relatives {
                x: iq
              }
              relatives {
                x: name
              }
            },
            human {
              relatives {
                iq
              }
            }
          }
        "# , & [RuleError :: new (& error_message ("relatives" , & Nested (vec ! [ConflictReason ("x" . into () , Message ("iq and name are different fields" . into ()) ,)]) ,) , & [SourcePosition :: new (47 , 3 , 14) , SourcePosition :: new (75 , 4 , 16) , SourcePosition :: new (111 , 6 , 14) , SourcePosition :: new (139 , 7 , 16) ,] ,)] ,) ; } # [test] fn reports_deep_conflict_to_nearest_common_ancestor_in_fragments () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            human {
              ...F
            }
            human {
              ...F
            }
          }
          fragment F on Human {
            relatives {
              relatives {
                x: iq
              }
              relatives {
                x: name
              }
            },
            relatives {
              relatives {
                iq
              }
            }
          }
        "# , & [RuleError :: new (& error_message ("relatives" , & Nested (vec ! [ConflictReason ("x" . into () , Message ("iq and name are different fields" . into ()) ,)]) ,) , & [SourcePosition :: new (201 , 11 , 14) , SourcePosition :: new (229 , 12 , 16) , SourcePosition :: new (265 , 14 , 14) , SourcePosition :: new (293 , 15 , 16) ,] ,)] ,) ; } # [test] fn reports_deep_conflict_in_nested_fragments () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog {
              ...F
            }
            dog {
              ...I
            }
          }
          fragment F on Dog {
            x: name
            ...G
          }
          fragment G on Dog {
            y: barkVolume
          }
          fragment I on Dog {
            y: nickname
            ...J
          }
          fragment J on Dog {
            x: barks
          }
        "# , & [RuleError :: new (& error_message ("dog" , & Nested (vec ! [ConflictReason ("x" . into () , Message ("name and barks are different fields" . into ()) ,) , ConflictReason ("y" . into () , Message ("barkVolume and nickname are different fields" . into ()) ,) ,]) ,) , & [SourcePosition :: new (25 , 2 , 12) , SourcePosition :: new (169 , 10 , 12) , SourcePosition :: new (248 , 14 , 12) , SourcePosition :: new (76 , 5 , 12) , SourcePosition :: new (399 , 21 , 12) , SourcePosition :: new (316 , 17 , 12) ,] ,)] ,) ; } # [test] fn ignores_unknown_fragments () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
        {
          dog {
            name
          }
          ...Unknown
          ...Known
        }

        fragment Known on QueryRoot {
          dog {
            name
          }
          ...OtherUnknown
        }
        "# ,) ; } struct SomeBox ; struct StringBox ; struct IntBox ; struct NonNullStringBox1 ; struct NonNullStringBox1Impl ; struct NonNullStringBox2 ; struct NonNullStringBox2Impl ; struct Connection ; struct Edge ; struct Node ; struct QueryRoot ; impl < S > GraphQLType < S > for SomeBox where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("SomeBox")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < SomeBox > > (arcstr :: literal ! ("deepBox") , i) , registry . field :: < Option < String > > (arcstr :: literal ! ("unrelatedField") , i) , registry . field :: < Option < String > > (arcstr :: literal ! ("otherField") , i) ,] ; registry . build_interface_type :: < Self > (i , fields) . into_meta () } } impl < S > GraphQLValue < S > for SomeBox where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } impl < S > GraphQLType < S > for StringBox where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("StringBox")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < String > > (arcstr :: literal ! ("scalar") , i) , registry . field :: < Option < StringBox > > (arcstr :: literal ! ("deepBox") , i) , registry . field :: < Option < String > > (arcstr :: literal ! ("unrelatedField") , i) , registry . field :: < Option < Vec < Option < StringBox > > > > (arcstr :: literal ! ("listStringBox") , i) , registry . field :: < Option < StringBox > > (arcstr :: literal ! ("stringBox") , i) , registry . field :: < Option < IntBox > > (arcstr :: literal ! ("intBox") , i) ,] ; registry . build_object_type :: < Self > (i , fields) . interfaces (& [registry . get_type :: < SomeBox > (i)]) . into_meta () } } impl < S > GraphQLValue < S > for StringBox where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } impl < S > GraphQLType < S > for IntBox where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("IntBox")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < i32 > > (arcstr :: literal ! ("scalar") , i) , registry . field :: < Option < IntBox > > (arcstr :: literal ! ("deepBox") , i) , registry . field :: < Option < String > > (arcstr :: literal ! ("unrelatedField") , i) , registry . field :: < Option < Vec < Option < StringBox > > > > (arcstr :: literal ! ("listStringBox") , i) , registry . field :: < Option < StringBox > > (arcstr :: literal ! ("stringBox") , i) , registry . field :: < Option < IntBox > > (arcstr :: literal ! ("intBox") , i) ,] ; registry . build_object_type :: < Self > (i , fields) . interfaces (& [registry . get_type :: < SomeBox > (i)]) . into_meta () } } impl < S > GraphQLValue < S > for IntBox where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } impl < S > GraphQLType < S > for NonNullStringBox1 where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("NonNullStringBox1")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < String > (arcstr :: literal ! ("scalar") , i)] ; registry . build_interface_type :: < Self > (i , fields) . into_meta () } } impl < S > GraphQLValue < S > for NonNullStringBox1 where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } impl < S > GraphQLType < S > for NonNullStringBox1Impl where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("NonNullStringBox1Impl")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < String > (arcstr :: literal ! ("scalar") , i) , registry . field :: < Option < SomeBox > > (arcstr :: literal ! ("deepBox") , i) , registry . field :: < Option < String > > (arcstr :: literal ! ("unrelatedField") , i) ,] ; registry . build_object_type :: < Self > (i , fields) . interfaces (& [registry . get_type :: < NonNullStringBox1 > (i) , registry . get_type :: < SomeBox > (i) ,]) . into_meta () } } impl < S > GraphQLValue < S > for NonNullStringBox1Impl where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } impl < S > GraphQLType < S > for NonNullStringBox2 where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("NonNullStringBox2")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < String > (arcstr :: literal ! ("scalar") , i)] ; registry . build_interface_type :: < Self > (i , fields) . into_meta () } } impl < S > GraphQLValue < S > for NonNullStringBox2 where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } impl < S > GraphQLType < S > for NonNullStringBox2Impl where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("NonNullStringBox2Impl")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < String > (arcstr :: literal ! ("scalar") , i) , registry . field :: < Option < SomeBox > > (arcstr :: literal ! ("deepBox") , i) , registry . field :: < Option < String > > (arcstr :: literal ! ("unrelatedField") , i) ,] ; registry . build_object_type :: < Self > (i , fields) . interfaces (& [registry . get_type :: < NonNullStringBox2 > (i) , registry . get_type :: < SomeBox > (i) ,]) . into_meta () } } impl < S > GraphQLValue < S > for NonNullStringBox2Impl where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } impl < S > GraphQLType < S > for Node where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("Node")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < ID > > (arcstr :: literal ! ("id") , i) , registry . field :: < Option < String > > (arcstr :: literal ! ("name") , i) ,] ; registry . build_object_type :: < Self > (i , fields) . into_meta () } } impl < S > GraphQLValue < S > for Node where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } impl < S > GraphQLType < S > for Edge where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("Edge")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < Node > > (arcstr :: literal ! ("node") , i)] ; registry . build_object_type :: < Self > (i , fields) . into_meta () } } impl < S > GraphQLValue < S > for Edge where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } impl < S > GraphQLType < S > for Connection where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("Connection")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < Vec < Option < Edge > > > > (arcstr :: literal ! ("edges") , i)] ; registry . build_object_type :: < Self > (i , fields) . into_meta () } } impl < S > GraphQLValue < S > for Connection where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } impl < S > GraphQLType < S > for QueryRoot where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("QueryRoot")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { registry . get_type :: < IntBox > (i) ; registry . get_type :: < StringBox > (i) ; registry . get_type :: < NonNullStringBox1Impl > (i) ; registry . get_type :: < NonNullStringBox2Impl > (i) ; let fields = & [registry . field :: < Option < SomeBox > > (arcstr :: literal ! ("someBox") , i) , registry . field :: < Option < Connection > > (arcstr :: literal ! ("connection") , i) ,] ; registry . build_object_type :: < Self > (i , fields) . into_meta () } } impl < S > GraphQLValue < S > for QueryRoot where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } } # [test] fn conflicting_return_types_which_potentially_overlap () { expect_fails_rule_with_schema :: < _ , EmptyMutation < () > , _ , _ , DefaultScalarValue > (QueryRoot , EmptyMutation :: new () , factory , r#"
            {
              someBox {
                ...on IntBox {
                  scalar
                }
                ...on NonNullStringBox1 {
                  scalar
                }
              }
            }
        "# , & [RuleError :: new (& error_message ("scalar" , & Message ("they return conflicting types Int and String!" . into ()) ,) , & [SourcePosition :: new (88 , 4 , 18) , SourcePosition :: new (173 , 7 , 18) ,] ,)] ,) ; } # [test] fn compatible_return_shapes_on_different_return_types () { expect_passes_rule_with_schema :: < _ , EmptyMutation < () > , EmptySubscription < () > , _ , _ , DefaultScalarValue , > (QueryRoot , EmptyMutation :: new () , EmptySubscription :: new () , factory , r#"
          {
            someBox {
              ... on SomeBox {
                deepBox {
                  unrelatedField
                }
              }
              ... on StringBox {
                deepBox {
                  unrelatedField
                }
              }
            }
          }
        "# ,) ; } # [test] fn disallows_differing_return_types_despite_no_overlap () { expect_fails_rule_with_schema :: < _ , EmptyMutation < () > , _ , _ , DefaultScalarValue > (QueryRoot , EmptyMutation :: new () , factory , r#"
            {
              someBox {
                ... on IntBox {
                  scalar
                }
                ... on StringBox {
                  scalar
                }
              }
            }
        "# , & [RuleError :: new (& error_message ("scalar" , & Message ("they return conflicting types Int and String" . into ()) ,) , & [SourcePosition :: new (89 , 4 , 18) , SourcePosition :: new (167 , 7 , 18) ,] ,)] ,) ; } # [test] fn reports_correctly_when_a_non_exclusive_follows_an_exclusive () { expect_fails_rule_with_schema :: < _ , EmptyMutation < () > , _ , _ , DefaultScalarValue > (QueryRoot , EmptyMutation :: new () , factory , r#"
            {
              someBox {
                ... on IntBox {
                  deepBox {
                    ...X
                  }
                }
              }
              someBox {
                ... on StringBox {
                  deepBox {
                    ...Y
                  }
                }
              }
              memoed: someBox {
                ... on IntBox {
                  deepBox {
                    ...X
                  }
                }
              }
              memoed: someBox {
                ... on StringBox {
                  deepBox {
                    ...Y
                  }
                }
              }
              other: someBox {
                ...X
              }
              other: someBox {
                ...Y
              }
            }
            fragment X on SomeBox {
              otherField
            }
            fragment Y on SomeBox {
              otherField: unrelatedField
            }
        "# , & [RuleError :: new (& error_message ("other" , & Nested (vec ! [ConflictReason ("otherField" . into () , Message ("otherField and unrelatedField are different fields" . into ()) ,)]) ,) , & [SourcePosition :: new (703 , 30 , 14) , SourcePosition :: new (889 , 38 , 14) , SourcePosition :: new (771 , 33 , 14) , SourcePosition :: new (964 , 41 , 14) ,] ,)] ,) ; } # [test] fn disallows_differing_return_type_nullability_despite_no_overlap () { expect_fails_rule_with_schema :: < _ , EmptyMutation < () > , _ , _ , DefaultScalarValue > (QueryRoot , EmptyMutation :: new () , factory , r#"
            {
              someBox {
                ... on NonNullStringBox1 {
                  scalar
                }
                ... on StringBox {
                  scalar
                }
              }
            }
        "# , & [RuleError :: new (& error_message ("scalar" , & Message ("they return conflicting types String! and String" . into ()) ,) , & [SourcePosition :: new (100 , 4 , 18) , SourcePosition :: new (178 , 7 , 18) ,] ,)] ,) ; } # [test] fn disallows_differing_return_type_list_despite_no_overlap () { expect_fails_rule_with_schema :: < _ , EmptyMutation < () > , _ , _ , DefaultScalarValue > (QueryRoot , EmptyMutation :: new () , factory , r#"
            {
              someBox {
                ... on IntBox {
                  box: listStringBox {
                    scalar
                  }
                }
                ... on StringBox {
                  box: stringBox {
                    scalar
                  }
                }
              }
            }
        "# , & [RuleError :: new (& error_message ("box" , & Message ("they return conflicting types [StringBox] and StringBox" . into ()) ,) , & [SourcePosition :: new (89 , 4 , 18) , SourcePosition :: new (228 , 9 , 18) ,] ,)] ,) ; expect_fails_rule_with_schema :: < _ , EmptyMutation < () > , _ , _ , DefaultScalarValue > (QueryRoot , EmptyMutation :: new () , factory , r#"
            {
              someBox {
                ... on IntBox {
                  box: stringBox {
                    scalar
                  }
                }
                ... on StringBox {
                  box: listStringBox {
                    scalar
                  }
                }
              }
            }
        "# , & [RuleError :: new (& error_message ("box" , & Message ("they return conflicting types StringBox and [StringBox]" . into ()) ,) , & [SourcePosition :: new (89 , 4 , 18) , SourcePosition :: new (224 , 9 , 18) ,] ,)] ,) ; } # [test] fn disallows_differing_subfields () { expect_fails_rule_with_schema :: < _ , EmptyMutation < () > , _ , _ , DefaultScalarValue > (QueryRoot , EmptyMutation :: new () , factory , r#"
            {
              someBox {
                ... on IntBox {
                  box: stringBox {
                    val: scalar
                    val: unrelatedField
                  }
                }
                ... on StringBox {
                  box: stringBox {
                    val: scalar
                  }
                }
              }
            }
        "# , & [RuleError :: new (& error_message ("val" , & Message ("scalar and unrelatedField are different fields" . into ()) ,) , & [SourcePosition :: new (126 , 5 , 20) , SourcePosition :: new (158 , 6 , 20) ,] ,)] ,) ; } # [test] fn disallows_differing_deep_return_types_despite_no_overlap () { expect_fails_rule_with_schema :: < _ , EmptyMutation < () > , _ , _ , DefaultScalarValue > (QueryRoot , EmptyMutation :: new () , factory , r#"
            {
              someBox {
                ... on IntBox {
                  box: stringBox {
                    scalar
                  }
                }
                ... on StringBox {
                  box: intBox {
                    scalar
                  }
                }
              }
            }
        "# , & [RuleError :: new (& error_message ("box" , & Nested (vec ! [ConflictReason ("scalar" . into () , Message ("they return conflicting types String and Int" . into ()) ,)]) ,) , & [SourcePosition :: new (89 , 4 , 18) , SourcePosition :: new (126 , 5 , 20) , SourcePosition :: new (224 , 9 , 18) , SourcePosition :: new (258 , 10 , 20) ,] ,)] ,) ; } # [test] fn allows_non_conflicting_overlapping_types () { expect_passes_rule_with_schema :: < _ , EmptyMutation < () > , EmptySubscription < () > , _ , _ , DefaultScalarValue , > (QueryRoot , EmptyMutation :: new () , EmptySubscription :: new () , factory , r#"
            {
              someBox {
                ... on IntBox {
                  scalar: unrelatedField
                }
                ... on StringBox {
                  scalar
                }
              }
            }
        "# ,) ; } # [test] fn same_wrapped_scalar_return_types () { expect_passes_rule_with_schema :: < _ , EmptyMutation < () > , EmptySubscription < () > , _ , _ , DefaultScalarValue , > (QueryRoot , EmptyMutation :: new () , EmptySubscription :: new () , factory , r#"
            {
              someBox {
                ...on NonNullStringBox1 {
                  scalar
                }
                ...on NonNullStringBox2 {
                  scalar
                }
              }
            }
        "# ,) ; } # [test] fn allows_inline_typeless_fragments () { expect_passes_rule_with_schema :: < _ , EmptyMutation < () > , EmptySubscription < () > , _ , _ , DefaultScalarValue , > (QueryRoot , EmptyMutation :: new () , EmptySubscription :: new () , factory , r#"
            {
              someBox {
                unrelatedField
              }
              ... {
                someBox {
                  unrelatedField
                }
              }
            }
        "# ,) ; } # [test] fn compares_deep_types_including_list () { expect_fails_rule_with_schema :: < _ , EmptyMutation < () > , _ , _ , DefaultScalarValue > (QueryRoot , EmptyMutation :: new () , factory , r#"
            {
              connection {
                ...edgeID
                edges {
                  node {
                    id: name
                  }
                }
              }
            }

            fragment edgeID on Connection {
              edges {
                node {
                  id
                }
              }
            }
        "# , & [RuleError :: new (& error_message ("edges" , & Nested (vec ! [ConflictReason ("node" . into () , Nested (vec ! [ConflictReason ("id" . into () , Message ("name and id are different fields" . into ()) ,)]) ,)]) ,) , & [SourcePosition :: new (84 , 4 , 16) , SourcePosition :: new (110 , 5 , 18) , SourcePosition :: new (137 , 6 , 20) , SourcePosition :: new (273 , 13 , 14) , SourcePosition :: new (297 , 14 , 16) , SourcePosition :: new (322 , 15 , 18) ,] ,)] ,) ; } # [test] fn ignores_unknown_types () { expect_passes_rule_with_schema :: < _ , EmptyMutation < () > , EmptySubscription < () > , _ , _ , DefaultScalarValue , > (QueryRoot , EmptyMutation :: new () , EmptySubscription :: new () , factory , r#"
            {
              someBox {
                ...on UnknownType {
                  scalar
                }
                ...on NonNullStringBox2 {
                  scalar
                }
              }
            }
        "# ,) ; } # [test] fn error_message_contains_hint_for_alias_conflict () { assert_eq ! (& error_message ("x" , & Message ("a and b are different fields" . into ())) , "Fields \"x\" conflict because a and b are different fields. Use \
             different aliases on the fields to fetch both if this \
             was intentional") ; } }
};
}
