macro_rules! deps {
    () => {
        ShouldEmit!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl ShouldEmit { pub (crate) fn emit_err (& self , diag : Diag < '_ >) -> ErrorGuaranteed { match self { ShouldEmit :: EarlyFatal { .. } if diag . level () == Level :: DelayedBug => diag . emit () , ShouldEmit :: EarlyFatal { .. } => diag . upgrade_to_fatal () . emit () , ShouldEmit :: ErrorsAndLints => diag . emit () , ShouldEmit :: Nothing => diag . delay_as_bug () , } } }
    };
}

impl_286!();