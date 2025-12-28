macro_rules! deps {
    () => {
        HeapVisitor!();
        Hir!();
        Result!();
        Visitor!();
    };
}

macro_rules! visit {
    () => {
        deps!();
        # [doc = " Executes an implementation of `Visitor` in constant stack space."] # [doc = ""] # [doc = " This function will visit every node in the given `Hir` while calling"] # [doc = " appropriate methods provided by the [`Visitor`] trait."] # [doc = ""] # [doc = " The primary use case for this method is when one wants to perform case"] # [doc = " analysis over an `Hir` without using a stack size proportional to the depth"] # [doc = " of the `Hir`. Namely, this method will instead use constant stack space,"] # [doc = " but will use heap space proportional to the size of the `Hir`. This may be"] # [doc = " desirable in cases where the size of `Hir` is proportional to end user"] # [doc = " input."] # [doc = ""] # [doc = " If the visitor returns an error at any point, then visiting is stopped and"] # [doc = " the error is returned."] pub fn visit < V : Visitor > (hir : & Hir , visitor : V) -> Result < V :: Output , V :: Err > { HeapVisitor :: new () . visit (hir , visitor) }
    };
}

visit!()