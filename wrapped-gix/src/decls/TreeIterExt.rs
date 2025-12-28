macro_rules! deps {
    () => {
        Sealed!();
        State!();
        Error!();
    };
}

macro_rules! TreeIterExt {
    () => {
        deps!();
        # [doc = " An extension trait for tree iterators"] pub trait TreeIterExt : Sealed { # [doc = " Traverse this tree with `state` being provided to potentially reuse allocations, and `find` being a function to lookup trees"] # [doc = " and turn them into iterators."] # [doc = ""] # [doc = " The `delegate` implements a way to store details about the traversal to allow paying only for what's actually used."] # [doc = " Since it is expected to store the operation result, _unit_ is returned."] fn traverse < StateMut , Find , V > (& self , state : StateMut , objects : Find , delegate : & mut V ,) -> Result < () , breadthfirst :: Error > where Find : gix_object :: Find , StateMut : BorrowMut < breadthfirst :: State > , V : gix_traverse :: tree :: Visit ; }
    };
}

TreeIterExt!()