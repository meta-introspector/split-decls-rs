macro_rules! ProcMacroServerChoice {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] pub enum ProcMacroServerChoice { Sysroot , Explicit (AbsPathBuf) , None , }
    };
}

ProcMacroServerChoice!()