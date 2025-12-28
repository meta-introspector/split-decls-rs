macro_rules! TraitBoundModifiers {
    () => {
        # [doc = " The modifiers on a trait bound."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug , HashStable_Generic)] pub struct TraitBoundModifiers { pub constness : BoundConstness , pub polarity : BoundPolarity , }
    };
}

TraitBoundModifiers!();