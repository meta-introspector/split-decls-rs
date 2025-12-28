macro_rules! deps {
    () => {
        Notify!();
        Arc!();
        Mutex!();
        Thread!();
    };
}

macro_rules! JoinHandle {
    () => {
        deps!();
        # [doc = " Mock implementation of `std::thread::JoinHandle`."] pub struct JoinHandle < T > { result : Arc < Mutex < Option < std :: thread :: Result < T > > > > , notify : rt :: Notify , thread : Thread , }
    };
}

JoinHandle!()