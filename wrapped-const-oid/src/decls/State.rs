macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [doc = " Current state of the encoder."] # [derive (Debug)] enum State { # [doc = " Initial state - no arcs yet encoded."] Initial , # [doc = " First arc has been supplied and stored as the wrapped [`Arc`]."] FirstArc (Arc) , # [doc = " Encoding base 128 body of the OID."] Body , }
    };
}

State!();