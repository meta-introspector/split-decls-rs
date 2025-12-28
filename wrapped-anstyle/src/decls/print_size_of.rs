macro_rules! deps {
    () => {
        StyleDisplay!();
        Style!();
    };
}

macro_rules! print_size_of {
    () => {
        deps!();
        # [test] # [cfg (feature = "std")] fn print_size_of () { use core :: mem :: size_of ; dbg ! (size_of ::< Style > ()) ; dbg ! (size_of ::< StyleDisplay > ()) ; }
    };
}

print_size_of!();