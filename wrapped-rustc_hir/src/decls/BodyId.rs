macro_rules! BodyId {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug , HashStable_Generic)] pub struct BodyId { pub hir_id : HirId , }
    };
}

BodyId!()