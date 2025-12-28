macro_rules! SegmentCommand {
    () => {
        struct SegmentCommand { cmdsize : u32 , segname : [u8 ; 16] , vmaddr : u64 , vmsize : u64 , fileoff : u64 , filesize : u64 , maxprot : u32 , initprot : u32 , nsects : u32 , flags : u32 , }
    };
}

SegmentCommand!();