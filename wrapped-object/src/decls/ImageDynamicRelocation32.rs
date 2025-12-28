macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageDynamicRelocation32 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageDynamicRelocation32 { pub symbol : U32 < LE > , pub base_reloc_size : U32 < LE > , }
    };
}

ImageDynamicRelocation32!()