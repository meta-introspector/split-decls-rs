macro_rules! AsmClobberNoReg {
    () => {
        pub (crate) struct AsmClobberNoReg { pub (crate) spans : Vec < Span > , pub (crate) clobbers : Vec < Span > , }
    };
}

AsmClobberNoReg!()