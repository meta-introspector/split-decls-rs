macro_rules! R_PPC_TOC16 {
    () => {
        # [doc = " This is a phony reloc to handle any old fashioned TOC16 references that may"] # [doc = " still be in object files."] pub const R_PPC_TOC16 : u32 = 255 ;
    };
}

R_PPC_TOC16!()