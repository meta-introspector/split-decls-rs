macro_rules! deps {
    () => {
        Waiter!();
    };
}

macro_rules! impl_1285 {
    () => {
        deps!();
        impl Waiter { fn register (& mut self , waker : & Waker) { match self { Self :: Waiting (w) if waker . will_wake (w) => { } _ => * self = Self :: Waiting (waker . clone ()) , } } fn wake (& mut self) { match mem :: replace (self , Self :: Woken) { Self :: Waiting (waker) => waker . wake () , Self :: Woken => { } } } }
    };
}

impl_1285!()