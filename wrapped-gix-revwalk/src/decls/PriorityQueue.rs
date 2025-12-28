macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! PriorityQueue {
    () => {
        deps!();
        # [doc = " A utility type implementing a queue which can be used to automatically sort data by its time in ascending order."] # [doc = ""] # [doc = " Note that the performance of this queue is very relevant to overall algorithm performance of many graph-walking algorithms,"] # [doc = " and as it stands our implementation is about 6% slower in practice, probably also depending on the size of the stored data."] # [derive (Default)] pub struct PriorityQueue < K : Ord , T > (std :: collections :: BinaryHeap < queue :: Item < K , T > >) ;
    };
}

PriorityQueue!()