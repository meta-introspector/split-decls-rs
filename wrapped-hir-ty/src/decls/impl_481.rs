macro_rules! deps {
    () => {
        BoundsFormattingCtx!();
        AliasTy!();
        HirFormatter!();
        DisplayLifetime!();
    };
}

macro_rules! impl_481 {
    () => {
        deps!();
        impl < 'db > HirFormatter < '_ , 'db > { fn start_location_link (& mut self , location : ModuleDefId) { self . fmt . start_location_link (location) ; } fn end_location_link (& mut self) { self . fmt . end_location_link () ; } fn format_bounds_with < T , F : FnOnce (& mut Self) -> T > (& mut self , target : AliasTy < 'db > , format_bounds : F ,) -> T { match self . bounds_formatting_ctx { BoundsFormattingCtx :: Entered { ref mut projection_tys_met } => { projection_tys_met . insert (target) ; format_bounds (self) } BoundsFormattingCtx :: Exited => { let mut projection_tys_met = FxHashSet :: default () ; projection_tys_met . insert (target) ; self . bounds_formatting_ctx = BoundsFormattingCtx :: Entered { projection_tys_met } ; let res = format_bounds (self) ; self . bounds_formatting_ctx = BoundsFormattingCtx :: Exited ; res } } } fn render_region (& self , lifetime : Region < 'db >) -> bool { match self . display_lifetimes { DisplayLifetime :: Always => true , DisplayLifetime :: OnlyStatic => matches ! (lifetime . kind () , RegionKind :: ReStatic) , DisplayLifetime :: OnlyNamed => { matches ! (lifetime . kind () , RegionKind :: ReEarlyParam (_)) } DisplayLifetime :: OnlyNamedOrStatic => { matches ! (lifetime . kind () , RegionKind :: ReStatic | RegionKind :: ReEarlyParam (_)) } DisplayLifetime :: Never => false , } } }
    };
}

impl_481!()