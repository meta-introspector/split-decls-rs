macro_rules! deps {
    () => {
        MemoryMap!();
        TraitEnvironment!();
        HirDisplayError!();
        TypingMode!();
        HirFormatter!();
    };
}

macro_rules! render_const_scalar {
    () => {
        deps!();
        fn render_const_scalar < 'db > (f : & mut HirFormatter < '_ , 'db > , b : & [u8] , memory_map : & MemoryMap < 'db > , ty : Ty < 'db > ,) -> Result < () , HirDisplayError > { let trait_env = TraitEnvironment :: empty (f . krate ()) ; let infcx = f . interner . infer_ctxt () . build (TypingMode :: PostAnalysis) ; let ty = infcx . at (& ObligationCause :: new () , trait_env . env) . deeply_normalize (ty) . unwrap_or (ty) ; render_const_scalar_inner (f , b , memory_map , ty , trait_env) }
    };
}

render_const_scalar!()