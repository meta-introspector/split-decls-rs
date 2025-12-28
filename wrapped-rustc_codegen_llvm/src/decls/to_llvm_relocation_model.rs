macro_rules! deps {
    () => {
        RelocModel!();
    };
}

macro_rules! to_llvm_relocation_model {
    () => {
        deps!();
        fn to_llvm_relocation_model (relocation_model : RelocModel) -> llvm :: RelocModel { match relocation_model { RelocModel :: Static => llvm :: RelocModel :: Static , RelocModel :: Pic | RelocModel :: Pie => llvm :: RelocModel :: PIC , RelocModel :: DynamicNoPic => llvm :: RelocModel :: DynamicNoPic , RelocModel :: Ropi => llvm :: RelocModel :: ROPI , RelocModel :: Rwpi => llvm :: RelocModel :: RWPI , RelocModel :: RopiRwpi => llvm :: RelocModel :: ROPI_RWPI , } }
    };
}

to_llvm_relocation_model!()