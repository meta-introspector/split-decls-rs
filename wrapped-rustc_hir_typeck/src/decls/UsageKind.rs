macro_rules! UsageKind {
    () => {
        enum UsageKind < 'tcx > { None , NonDefiningUse (OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >) , UnconstrainedHiddenType (OpaqueHiddenType < 'tcx >) , HasDefiningUse , }
    };
}

UsageKind!();