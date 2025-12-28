macro_rules! ParentIds {
    () => {
        # [doc = " The collection of parent ids we saw as part of the iteration."] # [doc = ""] # [doc = " Note that this list is truncated if [`Parents::First`] was used."] pub type ParentIds = SmallVec < gix_hash :: ObjectId , 1 > ;
    };
}

ParentIds!();