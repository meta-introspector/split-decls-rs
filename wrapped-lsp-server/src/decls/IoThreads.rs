macro_rules! IoThreads {
    () => {
        pub struct IoThreads { reader : thread :: JoinHandle < io :: Result < () > > , writer : thread :: JoinHandle < io :: Result < () > > , dropper : thread :: JoinHandle < () > , }
    };
}

IoThreads!()