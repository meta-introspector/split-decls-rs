macro_rules! PageTag {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] # [repr (u8)] pub enum PageTag { Events = 0 , StringData = 1 , StringIndex = 2 , }
    };
}

PageTag!();