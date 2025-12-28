macro_rules! deps {
    () => {
        QueueView!();
    };
}

macro_rules! Consumer {
    () => {
        deps!();
        # [doc = " A consumer; it can dequeue items from the queue."] # [doc = ""] # [doc = " **Note:** The consumer semantically owns the `head` pointer of the queue."] pub struct Consumer < 'a , T > { rb : & 'a QueueView < T > , }
    };
}

Consumer!()