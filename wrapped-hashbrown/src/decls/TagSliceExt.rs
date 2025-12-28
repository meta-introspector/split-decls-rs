macro_rules! deps {
    () => {
        Tag!();
    };
}

macro_rules! TagSliceExt {
    () => {
        deps!();
        # [doc = " Extension trait for slices of tags."] pub (crate) trait TagSliceExt { # [doc = " Fills the control with the given tag."] fn fill_tag (& mut self , tag : Tag) ; # [doc = " Clears out the control."] # [inline] fn fill_empty (& mut self) { self . fill_tag (Tag :: EMPTY) } }
    };
}

TagSliceExt!();