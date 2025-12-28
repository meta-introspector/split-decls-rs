macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! Update {
    () => {
        deps!();
        # [doc = " `Update` is an iterator that mutates the elements of an"] # [doc = " underlying iterator before they are yielded."] # [doc = ""] # [doc = " This struct is created by the [`update()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`update()`]: ParallelIterator::update()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct Update < I , F > { base : I , update_op : F , }
    };
}

Update!()