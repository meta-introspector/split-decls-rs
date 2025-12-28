macro_rules! MinScored {
    () => {
        # [doc = " `MinScored<K, T>` holds a score `K` and a scored object `T` in"] # [doc = " a pair for use with a `BinaryHeap`."] # [doc = ""] # [doc = " `MinScored` compares in reverse order by the score, so that we can"] # [doc = " use `BinaryHeap` as a min-heap to extract the score-value pair with the"] # [doc = " least score."] # [doc = ""] # [doc = " **Note:** `MinScored` implements a total order (`Ord`), so that it is"] # [doc = " possible to use float types as scores."] # [derive (Copy , Clone , Debug)] pub struct MinScored < K , T > (pub K , pub T) ;
    };
}

MinScored!();