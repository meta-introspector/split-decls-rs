macro_rules! DebugFile {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] enum DebugFile { Primary , Supplementary , Dwo , }
    };
}

DebugFile!();