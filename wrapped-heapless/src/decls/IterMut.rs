macro_rules! deps {
    () => {
        QueueView!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        # [doc = " An iterator over the items of a queue."] pub struct IterMut < 'a , T > { rb : & 'a QueueView < T > , index : usize , len : usize , }
    };
}

IterMut!();