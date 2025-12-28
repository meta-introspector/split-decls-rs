macro_rules! deps {
    () => {
        HistoryBufInner!();
    };
}

macro_rules! HistoryBuf {
    () => {
        deps!();
        # [doc = " A \"history buffer\", similar to a write-only ring buffer of fixed length."] # [doc = ""] # [doc = " This buffer keeps a fixed number of elements.  On write, the oldest element"] # [doc = " is overwritten. Thus, the buffer is useful to keep a history of values with"] # [doc = " some desired depth, and for example calculate a rolling average."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use heapless::HistoryBuf;"] # [doc = ""] # [doc = " // Initialize a new buffer with 8 elements."] # [doc = " let mut buf = HistoryBuf::<_, 8>::new();"] # [doc = ""] # [doc = " // Starts with no data"] # [doc = " assert_eq!(buf.recent(), None);"] # [doc = ""] # [doc = " buf.write(3);"] # [doc = " buf.write(5);"] # [doc = " buf.extend(&[4, 4]);"] # [doc = ""] # [doc = " // The most recent written element is a four."] # [doc = " assert_eq!(buf.recent(), Some(&4));"] # [doc = ""] # [doc = " // To access all elements in an unspecified order, use `as_slice()`."] # [doc = " for el in buf.as_slice() {"] # [doc = "     println!(\"{:?}\", el);"] # [doc = " }"] # [doc = ""] # [doc = " // Now we can prepare an average of all values, which comes out to 4."] # [doc = " let avg = buf.as_slice().iter().sum::<usize>() / buf.len();"] # [doc = " assert_eq!(avg, 4);"] # [doc = " ```"] pub type HistoryBuf < T , const N : usize > = HistoryBufInner < T , OwnedHistoryBufStorage < T , N > > ;
    };
}

HistoryBuf!();