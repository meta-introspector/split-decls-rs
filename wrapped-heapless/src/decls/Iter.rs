macro_rules! deps {
    () => {
        QueueView!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator over the items of a queue."] pub struct Iter < 'a , T > { rb : & 'a QueueView < T > , index : usize , len : usize , }
    };
}

Iter!();