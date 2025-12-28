macro_rules! LangCrateOrigin {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum LangCrateOrigin { Alloc , Core , ProcMacro , Std , Test , Other , }
    };
}

LangCrateOrigin!();