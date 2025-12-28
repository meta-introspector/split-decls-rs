macro_rules! deps {
    () => {
        ClassSetItem!();
        Ast!();
        ClassSetBinaryOp!();
    };
}

macro_rules! ClassInduct {
    () => {
        deps!();
        # [doc = " A representation of the inductive step when performing structural induction"] # [doc = " over a character class."] # [doc = ""] # [doc = " Note that there is no analogous explicit type for the inductive step for"] # [doc = " `Ast` nodes because the inductive step is just an `Ast`. For character"] # [doc = " classes, the inductive step can produce one of two possible child nodes:"] # [doc = " an item or a binary operation. (An item cannot be a binary operation"] # [doc = " because that would imply binary operations can be unioned in the concrete"] # [doc = " syntax, which is not possible.)"] enum ClassInduct < 'a > { Item (& 'a ast :: ClassSetItem) , BinaryOp (& 'a ast :: ClassSetBinaryOp) , }
    };
}

ClassInduct!()