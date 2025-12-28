macro_rules! deps {
    () => {
        Iter!();
        Drain!();
        ParallelDrainRange!();
    };
}

macro_rules! drain_guard {
    () => {
        deps!();
        mod drain_guard { use crate :: iter :: ParallelDrainRange ; use std :: mem ; use std :: ops :: RangeBounds ; # [doc = " A proxy for draining a collection by converting to a `Vec` and back."] # [doc = ""] # [doc = " This is used for draining `BinaryHeap` and `VecDeque`, which both have"] # [doc = " zero-allocation conversions to/from `Vec`, though not zero-cost:"] # [doc = " - `BinaryHeap` will heapify from `Vec`, but at least that will be empty."] # [doc = " - `VecDeque` has to shift items to offset 0 when converting to `Vec`."] # [allow (missing_debug_implementations)] pub (super) struct DrainGuard < 'a , T , C : From < Vec < T > > > { collection : & 'a mut C , vec : Vec < T > , } impl < 'a , T , C > DrainGuard < 'a , T , C > where C : Default + From < Vec < T > > , Vec < T > : From < C > , { pub (super) fn new (collection : & 'a mut C) -> Self { Self { vec : Vec :: from (mem :: take (collection)) , collection , } } } impl < 'a , T , C : From < Vec < T > > > Drop for DrainGuard < 'a , T , C > { fn drop (& mut self) { * self . collection = C :: from (mem :: take (& mut self . vec)) ; } } impl < 'a , T , C > ParallelDrainRange < usize > for & 'a mut DrainGuard < '_ , T , C > where T : Send , C : From < Vec < T > > , { type Iter = crate :: vec :: Drain < 'a , T > ; type Item = T ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { self . vec . par_drain (range) } } }
    };
}

drain_guard!()