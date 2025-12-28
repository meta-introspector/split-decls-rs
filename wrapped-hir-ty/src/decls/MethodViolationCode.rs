macro_rules! MethodViolationCode {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum MethodViolationCode { StaticMethod , ReferencesSelfInput , ReferencesSelfOutput , ReferencesImplTraitInTrait , AsyncFn , WhereClauseReferencesSelf , Generic , UndispatchableReceiver , }
    };
}

MethodViolationCode!()