macro_rules! ProcMacroDylibPath {
    () => {
        # [derive (Debug , Clone , Default , PartialEq , Eq)] pub enum ProcMacroDylibPath { Path (AbsPathBuf) , DylibNotFound , NotProcMacro , # [default] NotBuilt , }
    };
}

ProcMacroDylibPath!()