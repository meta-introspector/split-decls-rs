macro_rules! deps {
    () => {
        Locations!();
        NllTypeRelating!();
        UniverseInfo!();
        TypeChecker!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl < 'a , 'tcx > TypeChecker < 'a , 'tcx > { # [doc = " Adds sufficient constraints to ensure that `a R b` where `R` depends on `v`:"] # [doc = ""] # [doc = " - \"Covariant\" `a <: b`"] # [doc = " - \"Invariant\" `a == b`"] # [doc = " - \"Contravariant\" `a :> b`"] # [doc = ""] # [doc = " N.B., the type `a` is permitted to have unresolved inference"] # [doc = " variables, but not the type `b`."] # [instrument (skip (self) , level = "debug")] pub (super) fn relate_types (& mut self , a : Ty < 'tcx > , v : ty :: Variance , b : Ty < 'tcx > , locations : Locations , category : ConstraintCategory < 'tcx > ,) -> Result < () , NoSolution > { NllTypeRelating :: new (self , locations , category , UniverseInfo :: relate (a , b) , v) . relate (a , b) ? ; Ok (()) } # [doc = " Add sufficient constraints to ensure `a == b`. See also [Self::relate_types]."] pub (super) fn eq_args (& mut self , a : ty :: GenericArgsRef < 'tcx > , b : ty :: GenericArgsRef < 'tcx > , locations : Locations , category : ConstraintCategory < 'tcx > ,) -> Result < () , NoSolution > { NllTypeRelating :: new (self , locations , category , UniverseInfo :: other () , ty :: Invariant) . relate (a , b) ? ; Ok (()) } }
    };
}

impl_465!();