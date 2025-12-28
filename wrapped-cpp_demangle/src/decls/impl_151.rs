macro_rules! deps {
    () => {
        CtorDtorName!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl CtorDtorName { fn inheriting_mut (& mut self) -> & mut Option < TypeHandle > { match self { CtorDtorName :: CompleteConstructor (ref mut inheriting) | CtorDtorName :: BaseConstructor (ref mut inheriting) | CtorDtorName :: CompleteAllocatingConstructor (ref mut inheriting) | CtorDtorName :: MaybeInChargeConstructor (ref mut inheriting) => inheriting , CtorDtorName :: DeletingDestructor | CtorDtorName :: CompleteDestructor | CtorDtorName :: BaseDestructor | CtorDtorName :: MaybeInChargeDestructor => unreachable ! () , } } }
    };
}

impl_151!()