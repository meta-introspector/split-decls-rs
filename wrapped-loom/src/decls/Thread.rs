macro_rules! deps {
    () => {
        ThreadId!();
    };
}

macro_rules! Thread {
    () => {
        deps!();
        # [doc = " Mock implementation of `std::thread::Thread`."] # [derive (Clone , Debug)] pub struct Thread { id : ThreadId , name : Option < String > , }
    };
}

Thread!()