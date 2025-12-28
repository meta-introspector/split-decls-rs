macro_rules! GenericArgsProhibited {
    () => {
        # [derive (Debug)] pub struct GenericArgsProhibited { pub args : InFile < AstPtr < Either < ast :: GenericArgList , ast :: ParenthesizedArgList > > > , pub reason : GenericArgsProhibitedReason , }
    };
}

GenericArgsProhibited!();