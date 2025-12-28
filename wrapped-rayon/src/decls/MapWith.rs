macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! MapWith {
    () => {
        deps!();
        # [doc = " `MapWith` is an iterator that transforms the elements of an underlying iterator."] # [doc = ""] # [doc = " This struct is created by the [`map_with()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`map_with()`]: ParallelIterator::map_with()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct MapWith < I , T , F > { base : I , item : T , map_op : F , }
    };
}

MapWith!();