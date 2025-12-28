macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! follow {
    () => {
        deps!();
        # [doc = ""] pub mod follow { # [doc = ""] pub mod to_object { # [doc = " The error returned by [`Reference::follow_to_object(…)`](crate::Reference::follow_to_object())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FollowToObject (# [from] gix_ref :: peel :: to_object :: Error) , # [error (transparent)] PackedRefsOpen (# [from] gix_ref :: packed :: buffer :: open :: Error) , } } }
    };
}

follow!()