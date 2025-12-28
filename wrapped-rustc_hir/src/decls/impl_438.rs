macro_rules! deps {
    () => {
        Item!();
        EnumerateAndAdjust!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl < I > Iterator for EnumerateAndAdjust < I > where I : Iterator , { type Item = (usize , < I as Iterator > :: Item) ; fn next (& mut self) -> Option < (usize , < I as Iterator > :: Item) > { self . enumerate . next () . map (| (i , elem) | (if i < self . gap_pos { i } else { i + self . gap_len } , elem)) } fn size_hint (& self) -> (usize , Option < usize >) { self . enumerate . size_hint () } }
    };
}

impl_438!();