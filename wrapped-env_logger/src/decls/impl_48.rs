macro_rules! deps {
    () => {
        Formatter!();
        Timestamp!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl fmt :: Debug for Timestamp { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [doc = " A `Debug` wrapper for `Timestamp` that uses the `Display` implementation."] struct TimestampValue < 'a > (& 'a Timestamp) ; impl fmt :: Debug for TimestampValue < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } } f . debug_tuple ("Timestamp") . field (& TimestampValue (self)) . finish () } }
    };
}

impl_48!();