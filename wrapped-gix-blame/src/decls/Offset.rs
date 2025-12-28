macro_rules! Offset {
    () => {
        # [doc = " Describes the offset of a particular hunk relative to the *Blamed File*."] # [derive (Clone , Copy , Debug , PartialEq)] pub enum Offset { # [doc = " The amount of lines to add."] Added (u32) , # [doc = " The amount of lines to remove."] Deleted (u32) , }
    };
}

Offset!();