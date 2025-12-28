macro_rules! ImagePrologueDynamicRelocationHeader {
    () => {
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImagePrologueDynamicRelocationHeader { pub prologue_byte_count : u8 , }
    };
}

ImagePrologueDynamicRelocationHeader!()