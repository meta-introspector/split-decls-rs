macro_rules! Cycle {
    () => {
        # [derive (Default , Debug , Clone)] struct Cycle { cycle_fn : Option < (syn :: Ident , Path) > , cycle_initial : Option < (syn :: Ident , Path) > , cycle_result : Option < (syn :: Ident , Path) > , }
    };
}

Cycle!()