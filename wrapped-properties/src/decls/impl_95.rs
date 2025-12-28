macro_rules! deps {
    () => {
        GeneralCategoryOutOfBoundsError!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl TryFrom < u8 > for GeneralCategory { type Error = GeneralCategoryOutOfBoundsError ; # [doc = " Construct this [`GeneralCategory`] from an integer, returning"] # [doc = " an error if it is out of bounds"] fn try_from (val : u8) -> Result < Self , GeneralCategoryOutOfBoundsError > { GeneralCategory :: new_from_u8 (val) . ok_or (GeneralCategoryOutOfBoundsError) } }
    };
}

impl_95!();