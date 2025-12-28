macro_rules! deps {
    () => {
        TypeLocation!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl TypeLocation { pub (crate) fn complete_lifetimes (& self) -> bool { matches ! (self , TypeLocation :: GenericArg { corresponding_param : Some (ast :: GenericParam :: LifetimeParam (_)) , .. }) } pub (crate) fn complete_consts (& self) -> bool { matches ! (self , TypeLocation :: GenericArg { corresponding_param : Some (ast :: GenericParam :: ConstParam (_)) , .. } | TypeLocation :: AssocConstEq) } pub (crate) fn complete_types (& self) -> bool { match self { TypeLocation :: GenericArg { corresponding_param : Some (param) , .. } => { matches ! (param , ast :: GenericParam :: TypeParam (_)) } TypeLocation :: AssocConstEq => false , TypeLocation :: AssocTypeEq => true , TypeLocation :: ImplTrait => false , _ => true , } } pub (crate) fn complete_self_type (& self) -> bool { self . complete_types () && ! matches ! (self , TypeLocation :: ImplTarget | TypeLocation :: ImplTrait) } }
    };
}

impl_111!()