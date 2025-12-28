macro_rules! RepetitionRange {
    () => {
        # [doc = " A range repetition operator."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum RepetitionRange { # [doc = " `{m}`"] Exactly (u32) , # [doc = " `{m,}`"] AtLeast (u32) , # [doc = " `{m,n}`"] Bounded (u32 , u32) , }
    };
}

RepetitionRange!()