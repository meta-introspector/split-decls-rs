macro_rules! deps {
    () => {
        Axis!();
    };
}

macro_rules! Range {
    () => {
        deps!();
        # [doc = " Axis range"] # [derive (Clone , Copy)] pub enum Range { # [doc = " Autoscale the axis"] Auto , # [doc = " Set the limits of the axis"] Limits (f64 , f64) , }
    };
}

Range!()