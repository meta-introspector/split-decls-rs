macro_rules! deps {
    () => {
        EnumerateAndAdjust!();
        DotDotPos!();
    };
}

macro_rules! EnumerateAndAdjustIterator {
    () => {
        deps!();
        pub trait EnumerateAndAdjustIterator { fn enumerate_and_adjust (self , expected_len : usize , gap_pos : hir :: DotDotPos ,) -> EnumerateAndAdjust < Self > where Self : Sized ; }
    };
}

EnumerateAndAdjustIterator!()