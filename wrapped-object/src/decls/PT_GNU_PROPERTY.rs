macro_rules! deps {
    () => {
        Segment!();
    };
}

macro_rules! PT_GNU_PROPERTY {
    () => {
        deps!();
        # [doc = " Segment containing `.note.gnu.property` section."] pub const PT_GNU_PROPERTY : u32 = 0x6474_e553 ;
    };
}

PT_GNU_PROPERTY!()