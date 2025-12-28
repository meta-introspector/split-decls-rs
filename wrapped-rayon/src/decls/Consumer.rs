macro_rules! deps {
    () => {
        Reducer!();
        Producer!();
        Folder!();
    };
}

macro_rules! Consumer {
    () => {
        deps!();
        # [doc = " A consumer is effectively a [generalized \"fold\" operation][fold],"] # [doc = " and in fact each consumer will eventually be converted into a"] # [doc = " [`Folder`]. What makes a consumer special is that, like a"] # [doc = " [`Producer`], it can be **split** into multiple consumers using"] # [doc = " the `split_at` method. When a consumer is split, it produces two"] # [doc = " consumers, as well as a **reducer**. The two consumers can be fed"] # [doc = " items independently, and when they are done the reducer is used to"] # [doc = " combine their two results into one. See [the `plumbing`"] # [doc = " README][r] for further details."] # [doc = ""] # [doc = " [r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md"] # [doc = " [fold]: Iterator::fold()"] pub trait Consumer < Item > : Send + Sized { # [doc = " The type of folder that this consumer can be converted into."] type Folder : Folder < Item , Result = Self :: Result > ; # [doc = " The type of reducer that is produced if this consumer is split."] type Reducer : Reducer < Self :: Result > ; # [doc = " The type of result that this consumer will ultimately produce."] type Result : Send ; # [doc = " Divide the consumer into two consumers, one processing items"] # [doc = " `0..index` and one processing items from `index..`. Also"] # [doc = " produces a reducer that can be used to reduce the results at"] # [doc = " the end."] fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) ; # [doc = " Convert the consumer into a folder that can consume items"] # [doc = " sequentially, eventually producing a final result."] fn into_folder (self) -> Self :: Folder ; # [doc = " Hint whether this `Consumer` would like to stop processing"] # [doc = " further items, e.g. if a search has been completed."] fn full (& self) -> bool ; }
    };
}

Consumer!()