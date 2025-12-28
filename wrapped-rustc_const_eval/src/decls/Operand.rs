macro_rules! deps {
    () => {
        MemPlace!();
        Immediate!();
    };
}

macro_rules! Operand {
    () => {
        deps!();
        # [doc = " An `Operand` is the result of computing a `mir::Operand`. It can be immediate,"] # [doc = " or still in memory. The latter is an optimization, to delay reading that chunk of"] # [doc = " memory and to avoid having to store arbitrary-sized data here."] # [derive (Copy , Clone , Debug)] pub (super) enum Operand < Prov : Provenance = CtfeProvenance > { Immediate (Immediate < Prov >) , Indirect (MemPlace < Prov >) , }
    };
}

Operand!();