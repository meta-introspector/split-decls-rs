macro_rules! SeqId {
    () => {
        # [doc = " A <seq-id> production encoding a base-36 positive number."] # [doc = ""] # [doc = " ```text"] # [doc = " <seq-id> ::= <0-9A-Z>+"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct SeqId (usize) ;
    };
}

SeqId!();