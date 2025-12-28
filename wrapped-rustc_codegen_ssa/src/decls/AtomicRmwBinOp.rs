macro_rules! AtomicRmwBinOp {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug)] pub enum AtomicRmwBinOp { AtomicXchg , AtomicAdd , AtomicSub , AtomicAnd , AtomicNand , AtomicOr , AtomicXor , AtomicMax , AtomicMin , AtomicUMax , AtomicUMin , }
    };
}

AtomicRmwBinOp!();