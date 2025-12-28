macro_rules! deps {
    () => {
        LLVMRustResult!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl LLVMRustResult { pub (crate) fn into_result (self) -> Result < () , () > { match self { LLVMRustResult :: Success => Ok (()) , LLVMRustResult :: Failure => Err (()) , } } }
    };
}

impl_493!()