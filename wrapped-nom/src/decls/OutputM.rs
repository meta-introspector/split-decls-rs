macro_rules! deps {
    () => {
        Mode!();
        IsStreaming!();
    };
}

macro_rules! OutputM {
    () => {
        deps!();
        # [doc = " Holds the parser execution modifiers: output [Mode], error [Mode] and"] # [doc = " streaming behaviour for input data"] pub struct OutputM < M : Mode , EM : Mode , S : IsStreaming > { m : PhantomData < M > , em : PhantomData < EM > , s : PhantomData < S > , }
    };
}

OutputM!();