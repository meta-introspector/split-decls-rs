macro_rules! StoreOnHeap {
    () => {
        # [doc = " Indicates that storage should be allocated on heap."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct StoreOnHeap ;
    };
}

StoreOnHeap!()