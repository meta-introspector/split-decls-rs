macro_rules! ValuePairs {
    () => {
        # [doc = " See the `error_reporting` module for more details."] # [derive (Clone , Copy , Debug , PartialEq , Eq , TypeFoldable , TypeVisitable)] pub enum ValuePairs < 'tcx > { Regions (ExpectedFound < ty :: Region < 'tcx > >) , Terms (ExpectedFound < ty :: Term < 'tcx > >) , Aliases (ExpectedFound < ty :: AliasTerm < 'tcx > >) , TraitRefs (ExpectedFound < ty :: TraitRef < 'tcx > >) , PolySigs (ExpectedFound < ty :: PolyFnSig < 'tcx > >) , ExistentialTraitRef (ExpectedFound < ty :: PolyExistentialTraitRef < 'tcx > >) , ExistentialProjection (ExpectedFound < ty :: PolyExistentialProjection < 'tcx > >) , }
    };
}

ValuePairs!();