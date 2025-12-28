macro_rules! deps {
    () => {
        BindingMode!();
        TypeMismatch!();
        CandidateId!();
        InferenceDiagnostic!();
        Adjustment!();
        InternedOpaqueTyId!();
        InternedClosureId!();
        FnTrait!();
        InferenceContext!();
    };
}

macro_rules! InferenceResult {
    () => {
        deps!();
        # [doc = " The result of type inference: A mapping from expressions and patterns to types."] # [doc = ""] # [doc = " When you add a field that stores types (including `Substitution` and the like), don't forget"] # [doc = " `resolve_completely()`'ing  them in `InferenceContext::resolve_all()`. Inference variables must"] # [doc = " not appear in the final inference result."] # [derive (Clone , PartialEq , Eq , Debug)] pub struct InferenceResult < 'db > { # [doc = " For each method call expr, records the function it resolves to."] method_resolutions : FxHashMap < ExprId , (FunctionId , GenericArgs < 'db >) > , # [doc = " For each field access expr, records the field it resolves to."] field_resolutions : FxHashMap < ExprId , Either < FieldId , TupleFieldId > > , # [doc = " For each struct literal or pattern, records the variant it resolves to."] variant_resolutions : FxHashMap < ExprOrPatId , VariantId > , # [doc = " For each associated item record what it resolves to"] assoc_resolutions : FxHashMap < ExprOrPatId , (CandidateId , GenericArgs < 'db >) > , # [doc = " Whenever a tuple field expression access a tuple field, we allocate a tuple id in"] # [doc = " [`InferenceContext`] and store the tuples substitution there. This map is the reverse of"] # [doc = " that which allows us to resolve a [`TupleFieldId`]s type."] tuple_field_access_types : FxHashMap < TupleId , Tys < 'db > > , # [doc = " During inference this field is empty and [`InferenceContext::diagnostics`] is filled instead."] diagnostics : Vec < InferenceDiagnostic < 'db > > , pub (crate) type_of_expr : ArenaMap < ExprId , Ty < 'db > > , # [doc = " For each pattern record the type it resolves to."] # [doc = ""] # [doc = " **Note**: When a pattern type is resolved it may still contain"] # [doc = " unresolved or missing subpatterns or subpatterns of mismatched types."] pub (crate) type_of_pat : ArenaMap < PatId , Ty < 'db > > , pub (crate) type_of_binding : ArenaMap < BindingId , Ty < 'db > > , pub (crate) type_of_opaque : FxHashMap < InternedOpaqueTyId , Ty < 'db > > , pub (crate) type_mismatches : FxHashMap < ExprOrPatId , TypeMismatch < 'db > > , # [doc = " Whether there are any type-mismatching errors in the result."] pub (crate) has_errors : bool , # [doc = " Interned `Error` type to return references to."] error_ty : Ty < 'db > , # [doc = " Stores the types which were implicitly dereferenced in pattern binding modes."] pub (crate) pat_adjustments : FxHashMap < PatId , Vec < Ty < 'db > > > , # [doc = " Stores the binding mode (`ref` in `let ref x = 2`) of bindings."] # [doc = ""] # [doc = " This one is tied to the `PatId` instead of `BindingId`, because in some rare cases, a binding in an"] # [doc = " or pattern can have multiple binding modes. For example:"] # [doc = " ```"] # [doc = " fn foo(mut slice: &[u32]) -> usize {"] # [doc = "     slice = match slice {"] # [doc = "         [0, rest @ ..] | rest => rest,"] # [doc = "     };"] # [doc = "     0"] # [doc = " }"] # [doc = " ```"] # [doc = " the first `rest` has implicit `ref` binding mode, but the second `rest` binding mode is `move`."] pub (crate) binding_modes : ArenaMap < PatId , BindingMode > , pub (crate) expr_adjustments : FxHashMap < ExprId , Box < [Adjustment < 'db >] > > , pub (crate) closure_info : FxHashMap < InternedClosureId , (Vec < CapturedItem < 'db > > , FnTrait) > , pub mutated_bindings_in_closure : FxHashSet < BindingId > , pub (crate) coercion_casts : FxHashSet < ExprId > , }
    };
}

InferenceResult!()