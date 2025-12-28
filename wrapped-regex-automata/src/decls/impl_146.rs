macro_rules! deps {
    () => {
        StateMut!();
        State!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        # [cfg (feature = "dfa-build")] impl < 'a > fmt :: Debug for StateMut < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let state = State { id : self . id , is_match : self . is_match , ntrans : self . ntrans , input_ranges : self . input_ranges , next : self . next , pattern_ids : self . pattern_ids , accel : self . accel , } ; fmt :: Debug :: fmt (& state , f) } }
    };
}

impl_146!();