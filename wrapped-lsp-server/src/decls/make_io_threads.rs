macro_rules! deps {
    () => {
        IoThreads!();
    };
}

macro_rules! make_io_threads {
    () => {
        deps!();
        pub (crate) fn make_io_threads (reader : thread :: JoinHandle < io :: Result < () > > , writer : thread :: JoinHandle < io :: Result < () > > , dropper : thread :: JoinHandle < () > ,) -> IoThreads { IoThreads { reader , writer , dropper } }
    };
}

make_io_threads!();