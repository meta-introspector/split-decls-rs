macro_rules! deps {
    () => {
        Crate!();
    };
}

macro_rules! ToolModule {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct ToolModule { krate : base_db :: Crate , idx : u32 , }
    };
}

ToolModule!();