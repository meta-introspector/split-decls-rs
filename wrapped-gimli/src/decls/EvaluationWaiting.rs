macro_rules! deps {
    () => {
        Register!();
        Reader!();
    };
}

macro_rules! EvaluationWaiting {
    () => {
        deps!();
        # [derive (Debug)] enum EvaluationWaiting < R : Reader > { Memory , Register { offset : i64 } , FrameBase { offset : i64 } , Tls , Cfa , AtLocation , EntryValue , ParameterRef , RelocatedAddress , IndexedAddress , TypedLiteral { value : R } , Convert , Reinterpret , }
    };
}

EvaluationWaiting!()