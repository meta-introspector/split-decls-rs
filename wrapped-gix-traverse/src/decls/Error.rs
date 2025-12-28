macro_rules! Error {
    () => {
        # [doc = " The error is part of the item returned by the [`breadthfirst()`](crate::tree::breadthfirst())  and"] # [doc = "[`depthfirst()`](crate::tree::depthfirst()) functions."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Find (# [from] gix_object :: find :: existing_iter :: Error) , # [error ("The delegate cancelled the operation")] Cancelled , # [error (transparent)] ObjectDecode (# [from] gix_object :: decode :: Error) , }
    };
}

Error!()