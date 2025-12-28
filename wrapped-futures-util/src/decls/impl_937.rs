macro_rules! deps {
    () => {
        Sink!();
        Flush!();
    };
}

macro_rules! impl_937 {
    () => {
        deps!();
        # [doc = " A future that completes when the sink has finished processing all"] # [doc = " pending requests."] # [doc = ""] # [doc = " The sink itself is returned after flushing is complete; this adapter is"] # [doc = " intended to be used when you want to stop sending to the sink until"] # [doc = " all current requests are processed."] impl < 'a , Si : Sink < Item > + Unpin + ? Sized , Item > Flush < 'a , Si , Item > { pub (super) fn new (sink : & 'a mut Si) -> Self { Self { sink , _phantom : PhantomData } } }
    };
}

impl_937!();