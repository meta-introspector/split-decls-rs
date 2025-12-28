macro_rules! deps {
    () => {
        UpmapFromRaFixture!();
    };
}

macro_rules! Indel {
    () => {
        deps!();
        # [doc = " `InsertDelete` -- a single \"atomic\" change to text"] # [doc = ""] # [doc = " Must not overlap with other `InDel`s"] # [derive (Debug , Clone , PartialEq , Eq , Hash , UpmapFromRaFixture)] pub struct Indel { pub insert : String , # [doc = " Refers to offsets in the original text"] pub delete : TextRange , }
    };
}

Indel!()