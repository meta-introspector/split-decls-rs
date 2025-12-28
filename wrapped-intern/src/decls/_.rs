macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        const _ : () = assert ! (align_of ::< Symbol > () == align_of ::< NonNull < () >> ()) ;
    };
}

_!();