macro_rules! DefaultReturner {
    () => {
        # [derive (Default)] # [doc (hidden)] pub struct DefaultReturner < O > (PhantomData < O >) ;
    };
}

DefaultReturner!()