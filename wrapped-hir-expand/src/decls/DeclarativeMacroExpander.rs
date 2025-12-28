macro_rules! DeclarativeMacroExpander {
    () => {
        # [doc = " Old-style `macro_rules` or the new macros 2.0"] # [derive (Debug , Clone , Eq , PartialEq)] pub struct DeclarativeMacroExpander { pub mac : mbe :: DeclarativeMacro , pub transparency : Transparency , edition : Edition , }
    };
}

DeclarativeMacroExpander!()