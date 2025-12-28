macro_rules! deps {
    () => {
        EntryMarker!();
        SubmissionQueue!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < E : EntryMarker > Debug for SubmissionQueue < '_ , E > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_list () ; let mut pos = self . head ; while pos != self . tail { let entry : & E = unsafe { & * self . queue . sqes . add ((pos & self . queue . ring_mask) as usize) } ; d . entry (& entry) ; pos = pos . wrapping_add (1) ; } d . finish () } }
    };
}

impl_151!()