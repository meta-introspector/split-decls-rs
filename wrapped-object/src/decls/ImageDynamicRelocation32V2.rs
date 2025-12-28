macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageDynamicRelocation32V2 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageDynamicRelocation32V2 { pub header_size : U32 < LE > , pub fixup_info_size : U32 < LE > , pub symbol : U32 < LE > , pub symbol_group : U32 < LE > , pub flags : U32 < LE > , }
    };
}

ImageDynamicRelocation32V2!();