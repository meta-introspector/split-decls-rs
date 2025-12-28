macro_rules! CloneTypeIdentifier {
    () => {
        # [doc = " The `<clone-type-identifier>` pseudo-terminal."] # [doc = ""] # [doc = " ```text"] # [doc = " <clone-type-identifier> ::= <unqualified source code identifier>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct CloneTypeIdentifier { start : usize , end : usize , }
    };
}

CloneTypeIdentifier!()