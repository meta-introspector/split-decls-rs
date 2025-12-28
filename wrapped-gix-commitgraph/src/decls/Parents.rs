macro_rules! deps {
    () => {
        ParentIteratorState!();
        Commit!();
    };
}

macro_rules! Parents {
    () => {
        deps!();
        # [doc = " An iterator over parents of a [`Commit`]."] pub struct Parents < 'a > { commit_data : Commit < 'a > , state : ParentIteratorState < 'a > , }
    };
}

Parents!();