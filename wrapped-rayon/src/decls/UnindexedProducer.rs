macro_rules! deps {
    () => {
        Folder!();
        Producer!();
        Split!();
    };
}

macro_rules! UnindexedProducer {
    () => {
        deps!();
        # [doc = " A variant on `Producer` which does not know its exact length or"] # [doc = " cannot represent it in a `usize`. These producers act like"] # [doc = " ordinary producers except that they cannot be told to split at a"] # [doc = " particular point. Instead, you just ask them to split 'somewhere'."] # [doc = ""] # [doc = " (In principle, `Producer` could extend this trait; however, it"] # [doc = " does not because to do so would require producers to carry their"] # [doc = " own length with them.)"] pub trait UnindexedProducer : Send + Sized { # [doc = " The type of item returned by this producer."] type Item ; # [doc = " Split midway into a new producer if possible, otherwise return `None`."] fn split (self) -> (Self , Option < Self >) ; # [doc = " Iterate the producer, feeding each element to `folder`, and"] # [doc = " stop when the folder is full (or all elements have been consumed)."] fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > ; }
    };
}

UnindexedProducer!()