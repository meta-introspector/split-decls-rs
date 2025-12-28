macro_rules! deps {
    () => {
        TypeAscriptionTarget!();
    };
}

macro_rules! TypeLocation {
    () => {
        deps!();
        # [doc = " Original file ast nodes"] # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum TypeLocation { TupleField , TypeAscription (TypeAscriptionTarget) , # [doc = " Generic argument position e.g. `Foo<$0>`"] GenericArg { # [doc = " The generic argument list containing the generic arg"] args : Option < ast :: GenericArgList > , # [doc = " `Some(trait_)` if `trait_` is being instantiated with `args`"] of_trait : Option < hir :: Trait > , # [doc = " The generic parameter being filled in by the generic arg"] corresponding_param : Option < ast :: GenericParam > , } , # [doc = " Associated type equality constraint e.g. `Foo<Bar = $0>`"] AssocTypeEq , # [doc = " Associated constant equality constraint e.g. `Foo<X = $0>`"] AssocConstEq , TypeBound , ImplTarget , ImplTrait , Other , }
    };
}

TypeLocation!();