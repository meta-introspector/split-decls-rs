macro_rules! private {
    () => {
        mod private { use std :: ops ; use crate :: { TextRange , TextSize } ; pub trait SyntaxTextRange { fn start (& self) -> Option < TextSize > ; fn end (& self) -> Option < TextSize > ; } impl SyntaxTextRange for TextRange { fn start (& self) -> Option < TextSize > { Some (TextRange :: start (* self)) } fn end (& self) -> Option < TextSize > { Some (TextRange :: end (* self)) } } impl SyntaxTextRange for ops :: Range < TextSize > { fn start (& self) -> Option < TextSize > { Some (self . start) } fn end (& self) -> Option < TextSize > { Some (self . end) } } impl SyntaxTextRange for ops :: RangeFrom < TextSize > { fn start (& self) -> Option < TextSize > { Some (self . start) } fn end (& self) -> Option < TextSize > { None } } impl SyntaxTextRange for ops :: RangeTo < TextSize > { fn start (& self) -> Option < TextSize > { None } fn end (& self) -> Option < TextSize > { Some (self . end) } } impl SyntaxTextRange for ops :: RangeFull { fn start (& self) -> Option < TextSize > { None } fn end (& self) -> Option < TextSize > { None } } }
    };
}

private!()