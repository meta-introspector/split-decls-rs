macro_rules! Frozen {
    () => {
        # [doc = " `Frozen` is a graph wrapper."] # [doc = ""] # [doc = " The `Frozen` only allows shared access (read-only) to the"] # [doc = " underlying graph `G`, but it allows mutable access to its"] # [doc = " node and edge weights."] # [doc = ""] # [doc = " This is used to ensure immutability of the graph's structure"] # [doc = " while permitting weights to be both read and written."] # [doc = ""] # [doc = " See indexing implementations and the traits `Data` and `DataMap`"] # [doc = " for read-write access to the graph's weights."] pub struct Frozen < 'a , G : 'a > (& 'a mut G) ;
    };
}

Frozen!()