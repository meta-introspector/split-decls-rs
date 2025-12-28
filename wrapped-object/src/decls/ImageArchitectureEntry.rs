macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageArchitectureEntry {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageArchitectureEntry { # [doc = " RVA of instruction to fixup"] pub fixup_inst_rva : U32 < LE > , # [doc = " fixup instruction (see alphaops.h)"] pub new_inst : U32 < LE > , }
    };
}

ImageArchitectureEntry!();