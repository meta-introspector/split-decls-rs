macro_rules! RegionCtxt {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub (crate) enum RegionCtxt { Location (Location) , TyContext (TyContext) , Free (Symbol) , LateBound (Symbol) , Existential (Option < Symbol >) , Placeholder (Symbol) , Unknown , }
    };
}

RegionCtxt!()