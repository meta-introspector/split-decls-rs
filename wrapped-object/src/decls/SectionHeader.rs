macro_rules! SectionHeader {
    () => {
        pub struct SectionHeader { sectname : [u8 ; 16] , segname : [u8 ; 16] , addr : u64 , size : u64 , offset : u32 , align : u32 , reloff : u32 , nreloc : u32 , flags : u32 , }
    };
}

SectionHeader!()