macro_rules! deps {
    () => {
        MissingDebugImplementations!();
        ShadowedIntoIter!();
        MultipleSupertraitUpcastable!();
        CheckTransmutes!();
        IfLetRescope!();
        DefaultCouldBeDerived!();
        MissingDoc!();
        TypeLimits!();
        NonLocalDefinitions!();
        DanglingPointers!();
    };
}

macro_rules! macro_871 {
    () => {
        deps!();
        late_lint_methods ! (declare_combined_late_lint_pass , [BuiltinCombinedModuleLateLintPass , [ForLoopsOverFallibles : ForLoopsOverFallibles , DefaultCouldBeDerived : DefaultCouldBeDerived :: default () , DerefIntoDynSupertrait : DerefIntoDynSupertrait , DropForgetUseless : DropForgetUseless , ImproperCTypesLint : ImproperCTypesLint , InvalidFromUtf8 : InvalidFromUtf8 , VariantSizeDifferences : VariantSizeDifferences , PathStatements : PathStatements , LetUnderscore : LetUnderscore , InvalidReferenceCasting : InvalidReferenceCasting , ImplicitAutorefs : ImplicitAutorefs , UnusedResults : UnusedResults , UnitBindings : UnitBindings , NonUpperCaseGlobals : NonUpperCaseGlobals , NonShorthandFieldPatterns : NonShorthandFieldPatterns , UnusedAllocation : UnusedAllocation , MissingCopyImplementations : MissingCopyImplementations , PtrNullChecks : PtrNullChecks , MutableTransmutes : MutableTransmutes , TypeAliasBounds : TypeAliasBounds , TrivialConstraints : TrivialConstraints , TypeLimits : TypeLimits :: new () , NonSnakeCase : NonSnakeCase , InvalidNoMangleItems : InvalidNoMangleItems , UnreachablePub : UnreachablePub , ExplicitOutlivesRequirements : ExplicitOutlivesRequirements , InvalidValue : InvalidValue , DerefNullPtr : DerefNullPtr , UnstableFeatures : UnstableFeatures , UngatedAsyncFnTrackCaller : UngatedAsyncFnTrackCaller , ShadowedIntoIter : ShadowedIntoIter , DropTraitConstraints : DropTraitConstraints , DanglingPointers : DanglingPointers , NonPanicFmt : NonPanicFmt , NoopMethodCall : NoopMethodCall , EnumIntrinsicsNonEnums : EnumIntrinsicsNonEnums , InvalidAtomicOrdering : InvalidAtomicOrdering , AsmLabels : AsmLabels , OpaqueHiddenInferredBound : OpaqueHiddenInferredBound , MultipleSupertraitUpcastable : MultipleSupertraitUpcastable , MapUnitFn : MapUnitFn , MissingDebugImplementations : MissingDebugImplementations , MissingDoc : MissingDoc , AsyncClosureUsage : AsyncClosureUsage , AsyncFnInTrait : AsyncFnInTrait , NonLocalDefinitions : NonLocalDefinitions :: default () , ImplTraitOvercaptures : ImplTraitOvercaptures , IfLetRescope : IfLetRescope :: default () , StaticMutRefs : StaticMutRefs , UnqualifiedLocalImports : UnqualifiedLocalImports , CheckTransmutes : CheckTransmutes , LifetimeSyntax : LifetimeSyntax ,]]) ;
    };
}

macro_871!();