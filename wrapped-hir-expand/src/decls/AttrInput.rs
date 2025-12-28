macro_rules! AttrInput {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum AttrInput { # [doc = " `#[attr = \"string\"]`"] Literal (tt :: Literal) , # [doc = " `#[attr(subtree)]`"] TokenTree (tt :: TopSubtree) , }
    };
}

AttrInput!();