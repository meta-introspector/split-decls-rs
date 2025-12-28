macro_rules! LocalMutationIsAllowed {
    () => {
        # [doc = " When checking permissions for a place access, this flag is used to indicate that an immutable"] # [doc = " local place can be mutated."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum LocalMutationIsAllowed { Yes , # [doc = " We want use of immutable upvars to cause a \"write to immutable upvar\""] # [doc = " error, not an \"reassignment\" error."] ExceptUpvars , No , }
    };
}

LocalMutationIsAllowed!()