macro_rules! deps {
    () => {
        InternedStandardTypes!();
        HirDatabase!();
        BreakableContext!();
        TargetFeatureIsSafeInTarget!();
        Diverges!();
        MethodResolutionUnstableFeatures!();
        TargetFeatures!();
        InferenceResult!();
        Diagnostics!();
        InternedClosureId!();
        TyLoweringContext!();
        MirSpan!();
    };
}

macro_rules! InferenceContext {
    () => {
        deps!();
        # [doc = " The inference context contains all information needed during type inference."] # [derive (Clone , Debug)] pub (crate) struct InferenceContext < 'body , 'db > { pub (crate) db : & 'db dyn HirDatabase , pub (crate) owner : DefWithBodyId , pub (crate) body : & 'body Body , # [doc = " Generally you should not resolve things via this resolver. Instead create a TyLoweringContext"] # [doc = " and resolve the path via its methods. This will ensure proper error reporting."] pub (crate) resolver : Resolver < 'db > , target_features : OnceCell < (TargetFeatures , TargetFeatureIsSafeInTarget) > , pub (crate) unstable_features : MethodResolutionUnstableFeatures , pub (crate) edition : Edition , pub (crate) generic_def : GenericDefId , pub (crate) table : unify :: InferenceTable < 'db > , # [doc = " The traits in scope, disregarding block modules. This is used for caching purposes."] traits_in_scope : FxHashSet < TraitId > , pub (crate) result : InferenceResult < 'db > , tuple_field_accesses_rev : IndexSet < Tys < 'db > , std :: hash :: BuildHasherDefault < rustc_hash :: FxHasher > > , # [doc = " The return type of the function being inferred, the closure or async block if we're"] # [doc = " currently within one."] # [doc = ""] # [doc = " We might consider using a nested inference context for checking"] # [doc = " closures so we can swap all shared things out at once."] return_ty : Ty < 'db > , # [doc = " If `Some`, this stores coercion information for returned"] # [doc = " expressions. If `None`, this is in a context where return is"] # [doc = " inappropriate, such as a const expression."] return_coercion : Option < DynamicCoerceMany < 'db > > , # [doc = " The resume type and the yield type, respectively, of the coroutine being inferred."] resume_yield_tys : Option < (Ty < 'db > , Ty < 'db >) > , diverges : Diverges , breakables : Vec < BreakableContext < 'db > > , types : InternedStandardTypes < 'db > , # [doc = " Whether we are inside the pattern of a destructuring assignment."] inside_assignment : bool , deferred_cast_checks : Vec < CastCheck < 'db > > , current_captures : Vec < CapturedItemWithoutTy < 'db > > , # [doc = " A stack that has an entry for each projection in the current capture."] # [doc = ""] # [doc = " For example, in `a.b.c`, we capture the spans of `a`, `a.b`, and `a.b.c`."] # [doc = " We do that because sometimes we truncate projections (when a closure captures"] # [doc = " both `a.b` and `a.b.c`), and we want to provide accurate spans in this case."] current_capture_span_stack : Vec < MirSpan > , current_closure : Option < InternedClosureId > , # [doc = " Stores the list of closure ids that need to be analyzed before this closure. See the"] # [doc = " comment on `InferenceContext::sort_closures`"] closure_dependencies : FxHashMap < InternedClosureId , Vec < InternedClosureId > > , deferred_closures : FxHashMap < InternedClosureId , Vec < (Ty < 'db > , Ty < 'db > , Vec < Ty < 'db > > , ExprId) > > , diagnostics : Diagnostics < 'db > , }
    };
}

InferenceContext!();