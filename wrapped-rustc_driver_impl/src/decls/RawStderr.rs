macro_rules! RawStderr {
    () => {
        # [doc = " Unbuffered, unsynchronized writer to stderr."] # [doc = ""] # [doc = " Only acceptable because everything will end soon anyways."] struct RawStderr (()) ;
    };
}

RawStderr!()