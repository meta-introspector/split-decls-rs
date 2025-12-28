macro_rules! deps {
    () => {
        Mutex!();
        Notify!();
        Thread!();
        Arc!();
    };
}

macro_rules! JoinHandle {
    () => {
        deps!();
        # [doc = " Mock implementation of `std::thread::JoinHandle`."] pub struct JoinHandle < T > { result : Arc < Mutex < Option < std :: thread :: Result < T > > > > , notify : rt :: Notify , thread : Thread , }
    };
}

JoinHandle!();