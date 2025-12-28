macro_rules! SseMachine {
    () => {
        # [derive (Copy , Clone)] pub struct SseMachine < S3 , S4 , NI > (PhantomData < (S3 , S4 , NI) >) ;
    };
}

SseMachine!();