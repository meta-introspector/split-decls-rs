macro_rules! deps {
    () => {
        U16Bytes!();
        U32Bytes!();
    };
}

macro_rules! ImageEpilogueDynamicRelocationHeader {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageEpilogueDynamicRelocationHeader { pub epilogue_count : U32Bytes < LE > , pub epilogue_byte_count : u8 , pub branch_descriptor_element_size : u8 , pub branch_descriptor_count : U16Bytes < LE > , }
    };
}

ImageEpilogueDynamicRelocationHeader!()