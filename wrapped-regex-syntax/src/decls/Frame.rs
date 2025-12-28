macro_rules! deps {
    () => {
        Hir!();
        Alternation!();
        Repetition!();
        Capture!();
        Concat!();
    };
}

macro_rules! Frame {
    () => {
        deps!();
        # [doc = " Represents a single stack frame while performing structural induction over"] # [doc = " an `Hir`."] enum Frame < 'a > { # [doc = " A stack frame allocated just before descending into a repetition"] # [doc = " operator's child node."] Repetition (& 'a hir :: Repetition) , # [doc = " A stack frame allocated just before descending into a capture's child"] # [doc = " node."] Capture (& 'a hir :: Capture) , # [doc = " The stack frame used while visiting every child node of a concatenation"] # [doc = " of expressions."] Concat { # [doc = " The child node we are currently visiting."] head : & 'a Hir , # [doc = " The remaining child nodes to visit (which may be empty)."] tail : & 'a [Hir] , } , # [doc = " The stack frame used while visiting every child node of an alternation"] # [doc = " of expressions."] Alternation { # [doc = " The child node we are currently visiting."] head : & 'a Hir , # [doc = " The remaining child nodes to visit (which may be empty)."] tail : & 'a [Hir] , } , }
    };
}

Frame!()