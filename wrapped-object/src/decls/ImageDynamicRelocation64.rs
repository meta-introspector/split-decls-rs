macro_rules! deps {
    () => {
        U64!();
        U32!();
    };
}

macro_rules! ImageDynamicRelocation64 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageDynamicRelocation64 { pub symbol : U64 < LE > , pub base_reloc_size : U32 < LE > , }
    };
}

ImageDynamicRelocation64!()