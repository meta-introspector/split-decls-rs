macro_rules! deps {
    () => {
        Machine!();
        HasStaticRootDefId!();
        MemoryKind!();
        CompileTimeMachine!();
    };
}

macro_rules! other_220 {
    () => {
        deps!();
        pub trait CompileTimeMachine < 'tcx > = Machine < 'tcx , MemoryKind = const_eval :: MemoryKind , Provenance = CtfeProvenance , ExtraFnVal = ! , FrameExtra = () , AllocExtra = () , MemoryMap = FxIndexMap < AllocId , (MemoryKind < const_eval :: MemoryKind > , Allocation) > , > + HasStaticRootDefId ;
    };
}

other_220!();