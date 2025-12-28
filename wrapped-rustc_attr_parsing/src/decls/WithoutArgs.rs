macro_rules! deps {
    () => {
        Stage!();
        NoArgsAttributeParser!();
    };
}

macro_rules! WithoutArgs {
    () => {
        deps!();
        pub (crate) struct WithoutArgs < T : NoArgsAttributeParser < S > , S : Stage > (PhantomData < (S , T) >) ;
    };
}

WithoutArgs!();