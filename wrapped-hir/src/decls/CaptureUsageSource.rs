macro_rules! CaptureUsageSource {
    () => {
        # [derive (Debug)] pub struct CaptureUsageSource { is_ref : bool , source : InFile < AstPtr < Either < ast :: Expr , ast :: Pat > > > , }
    };
}

CaptureUsageSource!();