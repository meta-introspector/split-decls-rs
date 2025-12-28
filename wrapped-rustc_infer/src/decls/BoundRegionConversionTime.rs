macro_rules! BoundRegionConversionTime {
    () => {
        # [doc = " Times when we replace bound regions with existentials:"] # [derive (Clone , Copy , Debug)] pub enum BoundRegionConversionTime { # [doc = " when a fn is called"] FnCall , # [doc = " when two higher-ranked types are compared"] HigherRankedType , # [doc = " when projecting an associated type"] AssocTypeProjection (DefId) , }
    };
}

BoundRegionConversionTime!()