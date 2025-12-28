macro_rules! deps {
    () => {
        Reducer!();
        Consumer!();
        Once!();
    };
}

macro_rules! UnindexedConsumer {
    () => {
        deps!();
        # [doc = " A stateless consumer can be freely copied. These consumers can be"] # [doc = " used like regular consumers, but they also support a"] # [doc = " `split_off_left` method that does not take an index to split, but"] # [doc = " simply splits at some arbitrary point (`for_each`, for example,"] # [doc = " produces an unindexed consumer)."] pub trait UnindexedConsumer < I > : Consumer < I > { # [doc = " Splits off a \"left\" consumer and returns it. The `self`"] # [doc = " consumer should then be used to consume the \"right\" portion of"] # [doc = " the data. (The ordering matters for methods like find_first --"] # [doc = " values produced by the returned value are given precedence"] # [doc = " over values produced by `self`.) Once the left and right"] # [doc = " halves have been fully consumed, you should reduce the results"] # [doc = " with the result of `to_reducer`."] fn split_off_left (& self) -> Self ; # [doc = " Creates a reducer that can be used to combine the results from"] # [doc = " a split consumer."] fn to_reducer (& self) -> Self :: Reducer ; }
    };
}

UnindexedConsumer!();